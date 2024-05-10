use multiversx_sc::types::ManagedVec;

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

    #[event("sendFromLiquidtyEvent")]
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

    #[event("addTokensToWhitelistEvent")]
    fn add_tokens_to_whitelist_event(&self, #[indexed] tokens: &ManagedVec<TokenIdentifier>);

    #[event("removeTokensFromWhitelistEvent")]
    fn remove_tokens_from_whitelist_event(&self, #[indexed] tokens: &ManagedVec<TokenIdentifier>);

    #[event("setRelayerEvent")]
    fn set_relayer_event(&self, #[indexed] relayer: &ManagedAddress);

    #[event("addToLiquidtyEvent")]
    fn add_to_liquidity_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    #[event("removeFromLiquidtyEvent")]
    fn remove_from_liquidity_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    #[event("setWhitelistStateEvent")]
    fn set_whitelist_state_event(&self, #[indexed] state: &State);

    #[event("setDepositLimitsEvent")]
    fn set_deposit_limits_event(
        &self,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] minimum: &BigUint,
        #[indexed] maximum: &BigUint,
    );
}
