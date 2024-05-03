#![no_std]

use crate::errors::{
    ERR_CONTRACT_NOT_READY, ERR_NOT_ENOUGH_LIQUIDITY, ERR_NOT_PRIVILEGED,
    ERR_PAYMENT_AMOUNT_NOT_IN_ACCEPTED_RANGE, ERR_TOKEN_NOT_WHITELISTED,
};

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod errors;
pub mod events;
pub mod macros;
pub mod storage;

#[multiversx_sc::contract]
pub trait CoreMxBridgeSc:
    storage::StorageModule + config::ConfigModule + admin::AdminModule + events::EventsModule
{
    #[init]
    fn init(&self) {
        self.set_contract_state_inactive();
    }

    #[upgrade]
    fn upgrade(&self) {
        self.set_contract_state_inactive();
    }

    #[payable("*")]
    #[endpoint(sendToLiquidity)]
    fn send_to_liquidity(&self, extra_arguments: MultiValueEncoded<ManagedBuffer>) {
        let caller = self.blockchain().get_caller();
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        check_whitelist!(self, &caller, ERR_NOT_PRIVILEGED);
        let payment = self.call_value().single_esdt();

        require!(
            self.tokens_whitelist().contains(&payment.token_identifier),
            ERR_TOKEN_NOT_WHITELISTED
        );

        require!(
            self.minimum_deposit(&payment.token_identifier).get() <= payment.amount
                && payment.amount <= self.maximum_deposit(&payment.token_identifier).get(),
            ERR_PAYMENT_AMOUNT_NOT_IN_ACCEPTED_RANGE
        );

        let caller = self.blockchain().get_caller();

        self.send_to_liquidity_event(
            &payment.token_identifier,
            &payment.amount,
            &caller,
            extra_arguments.into_vec_of_buffers(),
        );

        self.liquidity(&payment.token_identifier)
            .update(|value| *value += payment.amount);
    }

    #[endpoint(sendFromLiquidity)]
    fn send_from_liquidity(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        address: ManagedAddress,
    ) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();
        require!(self.relayer().get() == caller, ERR_NOT_PRIVILEGED);

        require!(
            self.liquidity(&token_identifier).get() > amount,
            ERR_NOT_ENOUGH_LIQUIDITY
        );

        self.send_from_liquidity_event(&self.relayer().get(), &token_identifier, &amount, &address);

        self.send()
            .direct_esdt(&address, &token_identifier, 0u64, &amount);

        self.liquidity(&token_identifier)
            .update(|value| *value -= amount);
    }
}
