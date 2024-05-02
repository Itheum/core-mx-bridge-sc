use crate::{
    config::State,
    errors::{
        ERR_ALREADY_ACTIVE, ERR_ALREADY_INACTIVE, ERR_NOT_PRIVILEGED, ERR_TOKEN_NOT_WHITELISTED,
    },
    events, only_privileged, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule:
    crate::config::ConfigModule + storage::StorageModule + events::EventsModule
{
    #[endpoint(setContractStateActive)]
    fn set_contract_state_active(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            self.contract_state().get() == State::Inactive,
            ERR_ALREADY_ACTIVE
        );
        self.contract_state().set(State::Active);
        self.set_contract_state_event(&State::Active);
    }

    #[endpoint(setContractStateInactive)]
    fn set_contract_state_inactive(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            self.contract_state().get() == State::Active,
            ERR_ALREADY_INACTIVE
        );
        self.contract_state().set(State::Inactive);
        self.set_contract_state_event(&State::Inactive);
    }

    #[endpoint(addTokensToWhitelist)]
    fn add_tokens_to_whitelist(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.add_tokens_to_whitelist_event(&tokens.to_vec());
        for token in tokens.into_iter() {
            self.tokens_whitelist().insert(token);
        }
    }

    #[endpoint(removeTokensFromWhitelist)]
    fn remove_tokens_from_whitelist(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.remove_tokens_from_whitelist_event(&tokens.to_vec());
        for token in tokens.into_iter() {
            self.tokens_whitelist().swap_remove(&token);
        }
    }

    #[endpoint(setRelayer)]
    fn set_relayer(&self, relayer: ManagedAddress) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.set_relayer_event(&relayer);
        self.relayer().set(relayer);
    }

    #[payable("*")]
    #[endpoint(addToLiquidity)]
    fn add_to_liquidity(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let payment = self.call_value().single_esdt();

        require!(
            self.tokens_whitelist().contains(&payment.token_identifier),
            ERR_TOKEN_NOT_WHITELISTED
        );

        let caller = self.blockchain().get_caller();

        self.add_to_liquidity_event(&caller, &payment.token_identifier, &payment.amount);

        self.liquidity(&payment.token_identifier)
            .update(|value| *value += payment.amount);
    }

    #[endpoint(removeFromLiquidity)]
    fn remove_from_liquidity(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        require!(
            self.tokens_whitelist().contains(&token_identifier),
            ERR_TOKEN_NOT_WHITELISTED
        );

        let caller = self.blockchain().get_caller();

        self.remove_from_liquidity_event(&caller, &token_identifier, &amount);

        self.send()
            .direct_esdt(&caller, &token_identifier, 0u64, &amount);

        self.liquidity(&token_identifier)
            .update(|value| *value -= amount);
    }
}
