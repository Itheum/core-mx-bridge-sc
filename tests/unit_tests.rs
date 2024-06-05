use bridge_sc::bridge_sc::{
    ContractState, FIRST_USER_ADDRESS_EXPR, OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
};
use core_mx_bridge_sc::{
    config::{ConfigModule, State},
    storage::StorageModule,
    utils::UtilsModule,
};
use multiversx_sc::types::{BigUint, TokenIdentifier};
use multiversx_sc_scenario::{
    api::SingleTxApi,
    managed_address,
    scenario_model::{AddressValue, TxExpect},
};

use crate::bridge_sc::bridge_sc::{ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER};

mod bridge_sc;
mod endpoints;
#[test]
fn test_bridge_sc() {
    let bridge_sc = core_mx_bridge_sc::contract_obj::<SingleTxApi>();

    let mut check = bridge_sc.check_amount(&BigUint::from(10u64).pow(17), 18u32); // 0.1 tokens

    assert_eq!(check, false);

    check = bridge_sc.check_amount(&(BigUint::from(5u64) * BigUint::from(10u64).pow(17)), 18u32); // 0.5 tokens

    assert_eq!(check, false);

    check = bridge_sc.check_amount(&(BigUint::from(1u64) * BigUint::from(10u64).pow(18)), 18u32); // 1 token

    assert_eq!(check, true);

    check = bridge_sc.check_amount(
        &(BigUint::from(20u64) * BigUint::from(10u64).pow(18)),
        18u32,
    ); // 20 tokens

    assert_eq!(check, true);

    check = bridge_sc.check_amount(&BigUint::from(2111111111111111111u64), 18u32); // 2.1111 tokens

    assert_eq!(check, false);
}

#[test]
fn contract_is_ready_test() {
    let bridge_sc = core_mx_bridge_sc::contract_obj::<SingleTxApi>();

    let mut check = bridge_sc.contract_is_ready();

    assert_eq!(check, false);

    bridge_sc.contract_state().set(State::Active);

    check = bridge_sc.contract_is_ready();

    assert_eq!(check, false);

    bridge_sc.set_administrator(managed_address!(&AddressValue::from(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR
    )
    .to_address()));

    check = bridge_sc.contract_is_ready();

    assert_eq!(check, false);

    bridge_sc
        .relayer()
        .set(managed_address!(&AddressValue::from(
            ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR
        )
        .to_address()));

    check = bridge_sc.contract_is_ready();

    assert_eq!(check, false);

    bridge_sc.fee_value().set(BigUint::from(10u64));

    bridge_sc
        .tokens_whitelist()
        .insert(TokenIdentifier::from(ITHEUM_TOKEN_IDENTIFIER));

    check = bridge_sc.contract_is_ready();

    assert_eq!(check, false);

    bridge_sc
        .fee_collector()
        .set(managed_address!(&AddressValue::from(
            ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR
        )
        .to_address()));

    check = bridge_sc.contract_is_ready();

    assert_eq!(check, false);

    bridge_sc
        .wegld_contract_address()
        .set(managed_address!(&AddressValue::from(
            ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR
        )
        .to_address()));

    check = bridge_sc.contract_is_ready();

    assert_eq!(check, true);
}

#[test]
pub fn se_administrator_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();

    state.deploy();

    state.set_administrator(
        FIRST_USER_ADDRESS_EXPR,
        admin.clone(),
        Some(TxExpect::user_error(
            "str:Endpoint can only be called by owner",
        )),
    );

    state.set_administrator(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, admin.clone(), None);

    state.set_administrator(
        OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        admin.clone(),
        Some(TxExpect::user_error("str:Already in storage")),
    );
}
