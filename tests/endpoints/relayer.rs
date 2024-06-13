use multiversx_sc_scenario::imports::{CheckAccount, CheckStateStep, TxExpect};

use crate::bridge_sc::bridge_sc::{
    ContractState, ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, BRIDGE_CONTRACT_ADDRESS_EXPR,
    FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR,
    OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn relayer_test() {
    let mut state = ContractState::new();

    state.deploy();

    state.send_from_liquidity(
        RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        10_000,
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Contract not ready")),
    );

    state
        .default_deploy_and_set()
        .set_relayer_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None)
        .set_relayer_state_inactive(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None)
        .set_relayer_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None);

    state.send_from_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        10_000,
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.send_from_liquidity(
        RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        10_000,
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Not enough liquidity")),
    );

    state.add_to_liquidity(
        ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 10_000),
        None,
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BRIDGE_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000"),
        ));

    state.send_from_liquidity(
        RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        10_000,
        FIRST_USER_ADDRESS_EXPR,
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
            )
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "11_000"), // 10_000 + 1_000
            ),
    );
}
