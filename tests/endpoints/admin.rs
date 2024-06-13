use core_mx_bridge_sc::{
    config::{ProxyTrait as _, State},
    storage::ProxyTrait as _,
};
use multiversx_sc::{imports::SingleValue, types::BigUint};
use multiversx_sc_scenario::{
    managed_token_id,
    scenario_model::{CheckAccount, CheckStateStep, ScQueryStep, TxExpect},
};

use crate::bridge_sc::bridge_sc::{
    ContractState, ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, ANOTHER_TOKEN_IDENTIFIER,
    ANOTHER_TOKEN_IDENTIFIER_EXPR, BRIDGE_CONTRACT_ADDRESS_EXPR, FIRST_USER_ADDRESS_EXPR,
    ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR, OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn pause_unpause_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    state
        .deploy()
        .set_administrator(
            OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
            admin.clone(),
            Some(TxExpect::ok()),
        )
        .set_relayer(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, admin.clone(), None);

    state.check_contract_state(State::Inactive);

    state.set_contract_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None);
    state.check_contract_state(State::Active);

    state.set_contract_state_inactive(
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_contract_state_inactive(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));

    state.check_contract_state(State::Inactive);
}
#[test]

fn whitelist_active_inactive_test() {
    let mut state = ContractState::new();

    state.default_deploy_and_set();

    state.set_whitelist_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.whitelist_state())
            .expect_value(SingleValue::from(State::Active)),
    );

    state.set_whitelist_state_inactive(
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_whitelist_state_inactive(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.whitelist_state())
            .expect_value(SingleValue::from(State::Inactive)),
    );
}

#[test]
fn set_deposit_limits_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();

    state
        .deploy()
        .set_administrator(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, admin.clone(), None);

    state.set_deposit_limits(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        b"20",
        b"10",
        Some(TxExpect::user_error("str:Wrong values")),
    );

    state.set_deposit_limits(
        FIRST_USER_ADDRESS_EXPR,
        b"10",
        b"20",
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_deposit_limits(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, b"0", b"10", None);
}

#[test]
fn add_remove_token_from_whitelist() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();

    state
        .deploy()
        .set_administrator(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, admin, None);

    state.add_token_to_whitelist(
        FIRST_USER_ADDRESS_EXPR,
        ANOTHER_TOKEN_IDENTIFIER,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.add_token_to_whitelist(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ANOTHER_TOKEN_IDENTIFIER,
        None,
    );

    state.add_token_to_whitelist(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        None,
    );

    state.add_token_to_whitelist(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        Some(TxExpect::user_error("str:Token already in whitelist")),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.token_whitelist())
            .expect_value(SingleValue::from(managed_token_id!(
                ITHEUM_TOKEN_IDENTIFIER
            ))),
    );

    state.remove_token_from_whitelist(
        FIRST_USER_ADDRESS_EXPR,
        ANOTHER_TOKEN_IDENTIFIER,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.remove_token_from_whitelist(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.token_whitelist())
            .expect_value(SingleValue::from(managed_token_id!(b""))),
    );
}

#[test]
fn set_relayer_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    let relayer = state.relayer.clone();

    state
        .deploy()
        .set_administrator(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, admin, None);

    state.set_relayer(
        FIRST_USER_ADDRESS_EXPR,
        relayer.clone(),
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_relayer(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, relayer.clone(), None);
}

#[test]
fn add_remove_to_from_liquidity() {
    let mut state = ContractState::new();

    state.default_deploy_and_set();

    state.add_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (ANOTHER_TOKEN_IDENTIFIER_EXPR, 0u64, 10u64),
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.add_to_liquidity(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        (ANOTHER_TOKEN_IDENTIFIER_EXPR, 0u64, 10u64),
        Some(TxExpect::user_error("str:Token not whitelisted")),
    );

    state.add_to_liquidity(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 10_000_000),
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000_000"),
            )
            .put_account(
                ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
            ),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(
                state
                    .contract
                    .liquidity(managed_token_id!(ITHEUM_TOKEN_IDENTIFIER)),
            )
            .expect_value(SingleValue::from(BigUint::from(10_000_000u64))),
    );

    state.remove_from_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        10_000_000,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.remove_from_liquidity(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        10_000_000,
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000_000"),
            )
            .put_account(
                BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
            ),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(
                state
                    .contract
                    .liquidity(managed_token_id!(ITHEUM_TOKEN_IDENTIFIER)),
            )
            .expect_value(SingleValue::from(BigUint::from(0u64))),
    );
}
