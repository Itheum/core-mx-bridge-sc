use core_mx_bridge_sc::config::State;
use multiversx_sc_scenario::scenario_model::TxExpect;

use crate::bridge_sc::{
    bridge_sc::ContractState, ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, FIRST_USER_ADDRESS,
    OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn pause_unpause_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    state.deploy().set_administrator(
        OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        admin,
        Some(TxExpect::ok()),
    );

    state.check_contract_state(State::Inactive);

    state.set_contract_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None);
    state.check_contract_state(State::Active);

    state.set_contract_state_inactive(
        FIRST_USER_ADDRESS,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_contract_state_inactive(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));

    state.check_contract_state(State::Inactive);
}
