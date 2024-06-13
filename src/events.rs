use crate::config::State;

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("sendToLiquidityEvent")]
    fn send_to_liquidity_event(
        &self,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
        #[indexed] address: &ManagedAddress,
        #[indexed] destination_address: &ManagedBuffer,
        #[indexed] destination_signature: &ManagedBuffer,
    );

    #[event("addToWhitelist")]
    fn add_to_whitelist_event(&self, #[indexed] address: &ManagedAddress);

    #[event("removeFromWhitelist")]
    fn remove_from_whitelist_event(&self, #[indexed] address: &ManagedAddress);

    #[event("sendFromLiquidityEvent")]
    fn send_from_liquidity_event(
        &self,
        #[indexed] relayer: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
        #[indexed] address: &ManagedAddress,
    );

    #[event("setContractStateEvent")]
    fn set_contract_state_event(&self, #[indexed] state: &State);

    #[event("setAdministratorEvent")]
    fn set_administrator_event(&self, #[indexed] administrator: &ManagedAddress);

    #[event("setRelayerEvent")]
    fn set_relayer_event(&self, #[indexed] relayer: &ManagedAddress);

    #[event("addToLiquidityEvent")]
    fn add_to_liquidity_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    #[event("removeFromLiquidityEvent")]
    fn remove_from_liquidity_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    #[event("setWhitelistStateEvent")]
    fn set_whitelist_state_event(&self, #[indexed] state: &State);

    #[event("setDepositLimitsEvent")]
    fn set_deposit_limits_event(&self, #[indexed] minimum: &BigUint, #[indexed] maximum: &BigUint);

    #[event("addTokenToWhitelistEvent")]
    fn add_token_to_whitelist_event(&self, #[indexed] token: &TokenIdentifier, decimals: &u32);

    #[event("removeTokenFromWhitelistEvent")]
    fn remove_token_from_whitelist_event(&self, #[indexed] token: &TokenIdentifier);

    #[event("setFeeCollectorEvent")]
    fn set_fee_collector_event(&self, #[indexed] fee_collector: &ManagedAddress);

    #[event("setFeeValueEvent")]
    fn set_fee_value_event(&self, #[indexed] fee_value: &BigUint);

    #[event("setWegldTokenIdentifier")]
    fn set_wegld_token_identifier(&self, #[indexed] wegld_token_identifier: &TokenIdentifier);

    #[event("setRelayerStateEvent")]
    fn set_relayer_state_event(&self, #[indexed] state: &State);
}
