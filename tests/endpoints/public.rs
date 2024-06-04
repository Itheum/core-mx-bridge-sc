use multiversx_sc_scenario::imports::{
    Account, CheckAccount, CheckStateStep, ScCallStep, SetStateStep, TxExpect,
};

use crate::bridge_sc::bridge_sc::{
    ContractState, ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, ANOTHER_TOKEN_IDENTIFIER_EXPR,
    BRIDGE_CONTRACT_ADDRESS_EXPR, FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER,
    ITHEUM_TOKEN_IDENTIFIER_EXPR, OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
    OWNER_WEGLD_SWAP_CONTRACT_ADDRESS_EXPR, RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
    THIRD_USER_ADDRESS_EXPR, WEGLD_TOKEN_IDENTIFIER_EXPR,
};

use multiversx_sc_modules::pause::ProxyTrait;
use multiversx_wegld_swap_sc::ProxyTrait as _;

#[test]
fn send_to_bridge_test() {
    let mut state = ContractState::new();
    let first_user = state.first_user.clone();

    state.deploy();

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, "1_000"),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Contract not ready")),
    );

    state
        .default_deploy_and_set()
        .set_contract_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None);

    state.set_whitelist_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None);

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, "1_000"),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Address not whitelisted")),
    );

    state.add_to_whitelist(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR, first_user.clone(), None);

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, "1_000"),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Not a whole number")),
    );

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, "1_000"),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Not a whole number")),
    );

    state.world.set_state_step(
        SetStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            Account::new()
                .nonce(1)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_100_000_000_000_000_000"),
        ),
    );

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "1_000_000_000_000_000_000",
        ),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error(
            "str:Payment amount not in accepted range",
        )),
    );

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "1_000_000_000_000_000_000",
        ),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error(
            "str:Payment amount not in accepted range",
        )),
    );
    state.set_deposit_limits(
        OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        b"0",
        b"100000000000000000000", // 100 tokens
        None,
    );

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "1_100_000_000_000_000_000",
        ),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Not a whole number")),
    );

    state.send_to_liquidity(
        FIRST_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "1_000_000_000_000_000_000",
        ),
        vec![b"sol_address", b"sol_signature"],
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new()
                    .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000_000_000_000_000_000"),
            )
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                CheckAccount::new()
                    .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100_000_000_000_000_000"),
            ),
    );
}

#[test]
fn send_to_bridge_require_fee_test() {
    let mut state = ContractState::new();

    state
        .default_deploy_and_set()
        .deploy_wegld_swap()
        .set_contract_state_active(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, None)
        .set_fee_value(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR, 1_000u64, None);

    state.set_deposit_limits(
        OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        b"0",
        b"1000000000000000000000", // 1000 tokens
        None,
    );

    state.world.set_state_step(
        SetStateStep::new().put_account(
            THIRD_USER_ADDRESS_EXPR,
            Account::new()
                .nonce(1)
                .balance("1_000")
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100_000_000_000_000_000_000") // 100 tokens
                .esdt_balance(ANOTHER_TOKEN_IDENTIFIER_EXPR, "1_000"),
        ),
    );

    state.world.sc_call(
        ScCallStep::new()
            .from(OWNER_WEGLD_SWAP_CONTRACT_ADDRESS_EXPR)
            .call(state.wegld_swap_contract.unpause_endpoint())
            .expect(TxExpect::ok()),
    );

    state.world.sc_call(
        ScCallStep::new()
            .from(THIRD_USER_ADDRESS_EXPR)
            .egld_value("1_000")
            .call(state.wegld_swap_contract.wrap_egld())
            .expect(TxExpect::ok()),
    );

    state.send_to_liquidity(
        THIRD_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "1_000_000_000_000_000_000",
        ),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error(
            "str:incorrect number of ESDT transfers",
        )),
    );

    state.send_to_liquidity_with_fee(
        THIRD_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "10_000_000_000_000_000_000",
        ),
        (ANOTHER_TOKEN_IDENTIFIER_EXPR, 0u64, "1_000"),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Wrong fee token identifier")),
    );

    state.send_to_liquidity_with_fee(
        THIRD_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "10_000_000_000_000_000_000",
        ),
        (WEGLD_TOKEN_IDENTIFIER_EXPR, 0u64, "100"),
        vec![b"sol_address", b"sol_signature"],
        Some(TxExpect::user_error("str:Wrong values")),
    );

    state.send_to_liquidity_with_fee(
        THIRD_USER_ADDRESS_EXPR,
        (
            ITHEUM_TOKEN_IDENTIFIER_EXPR,
            0u64,
            "10_000_000_000_000_000_000",
        ),
        (WEGLD_TOKEN_IDENTIFIER_EXPR, 0u64, "1_000"),
        vec![b"sol_address", b"sol_signature"],
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new()
                    .balance("0")
                    .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000_000_000_000_000_000"),
            )
            .put_account(
                THIRD_USER_ADDRESS_EXPR,
                CheckAccount::new()
                    .balance("0")
                    .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "90_000_000_000_000_000_000"),
            )
            .put_account(
                RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().balance("1_000"), // fee
            ),
    );
}
