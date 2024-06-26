// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           33
// Async Callback (empty):               1
// Total number of exported functions:  36

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    core_mx_bridge_sc
    (
        init => init
        upgrade => upgrade
        sendToLiquidity => send_to_liquidity
        sendFromLiquidity => send_from_liquidity
        getLiquidity => liquidity
        getTokenDecimals => token_decimals
        getMinimumDeposit => minimum_deposit
        getMaximumDeposit => maximum_deposit
        getFeeCollector => fee_collector
        getFeeValue => fee_value
        getWegldTokenIdentifier => wegld_token_identifier
        setAdministrator => set_administrator
        getAdministrator => administrator
        getTokensWhitelist => token_whitelist
        getWhitelistState => whitelist_state
        getPublicState => public_state
        getRelayerState => relayer_state
        getRelayer => relayer
        setPublicStateActive => set_public_state_active
        setPublicStateInactive => set_public_state_inactive
        setRelayerStateActive => set_relayer_state_active
        setRelayerStateInactive => set_relayer_state_inactive
        setWhitelistStateActive => set_whitelist_state_active
        setWhitelistStateInactive => set_whitelist_state_inactive
        setDepositLimits => set_deposit_limits
        setFeeCollector => set_fee_collector
        setFeeValue => set_fee_value
        setWegldTokenIdentifier => set_wegld_contract_address
        addTokenToWhitelist => add_tokens_to_whitelist
        removeTokenFromWhitelist => remove_token_from_whitelist
        addToWhitelist => add_to_whitelist
        removeFromWhitelist => remove_from_whitelist
        setRelayer => set_relayer
        addToLiquidity => add_to_liquidity
        removeFromLiquidity => remove_from_liquidity
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
