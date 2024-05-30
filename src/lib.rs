#![no_std]

use crate::errors::{
    ERR_ADDRESS_NOT_WHITELISTED, ERR_CONTRACT_NOT_READY, ERR_NOT_ENOUGH_LIQUIDITY,
    ERR_NOT_PRIVILEGED, ERR_NOT_WHOLE_NUMBER, ERR_PAYMENT_AMOUNT_NOT_IN_ACCEPTED_RANGE,
    ERR_TOKEN_NOT_WHITELISTED, ERR_WRONG_VALUES,
};

use crate::proxies::wegld_proxy;

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod errors;
pub mod events;
pub mod macros;
pub mod proxies;
pub mod storage;
pub mod utils;
#[multiversx_sc::contract]
pub trait CoreMxBridgeSc:
    storage::StorageModule
    + config::ConfigModule
    + admin::AdminModule
    + events::EventsModule
    + utils::UtilsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {
        self.set_contract_state_inactive();
    }

    #[payable("*")]
    #[endpoint(sendToLiquidity)]
    fn send_to_liquidity(
        &self,
        destination_address: ManagedBuffer,
        destination_signature: ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        check_whitelist!(self, &caller, ERR_ADDRESS_NOT_WHITELISTED);

        let fee_value = self.fee_value().get();

        if fee_value == BigUint::zero() {
            let deposit = self.call_value().single_esdt();

            require!(
                self.tokens_whitelist().contains(&deposit.token_identifier),
                ERR_TOKEN_NOT_WHITELISTED
            );

            require!(
                self.check_amount(
                    &deposit.amount,
                    self.token_decimals(&deposit.token_identifier).get()
                ),
                ERR_NOT_WHOLE_NUMBER
            );

            require!(
                self.minimum_deposit(&deposit.token_identifier).get() <= deposit.amount
                    && deposit.amount <= self.maximum_deposit(&deposit.token_identifier).get(),
                ERR_PAYMENT_AMOUNT_NOT_IN_ACCEPTED_RANGE
            );

            self.send_to_liquidity_event(
                &deposit.token_identifier,
                &deposit.amount,
                &caller,
                &destination_address,
                &destination_signature,
            );

            self.liquidity(&deposit.token_identifier)
                .update(|value| *value += deposit.amount);
        } else {
            let [deposit, fee] = self.call_value().multi_esdt();

            require!(self.fee_value().get() == fee.amount, ERR_WRONG_VALUES);

            let wegld_token_identifier = self
                .tx()
                .to(&self.wegld_contract_address().get())
                .typed(wegld_proxy::EgldEsdtSwapProxy)
                .wrapped_egld_token_id()
                .returns(ReturnsResult)
                .sync_call();

            require!(
                fee.token_identifier == wegld_token_identifier,
                ERR_TOKEN_NOT_WHITELISTED
            );

            let back_transfers = self
                .tx()
                .to(&self.wegld_contract_address().get())
                .typed(wegld_proxy::EgldEsdtSwapProxy)
                .unwrap_egld()
                .returns(ReturnsBackTransfers)
                .sync_call();

            self.send().direct_egld(
                &self.fee_colector().get(),
                &back_transfers.total_egld_amount,
            );

            require!(
                self.tokens_whitelist().contains(&deposit.token_identifier),
                ERR_TOKEN_NOT_WHITELISTED
            );

            require!(
                self.check_amount(
                    &deposit.amount,
                    self.token_decimals(&deposit.token_identifier).get()
                ),
                ERR_NOT_WHOLE_NUMBER
            );

            require!(
                self.minimum_deposit(&deposit.token_identifier).get() <= deposit.amount
                    && deposit.amount <= self.maximum_deposit(&deposit.token_identifier).get(),
                ERR_PAYMENT_AMOUNT_NOT_IN_ACCEPTED_RANGE
            );

            self.send_to_liquidity_event(
                &deposit.token_identifier,
                &deposit.amount,
                &caller,
                &destination_address,
                &destination_signature,
            );

            self.liquidity(&deposit.token_identifier)
                .update(|value| *value += deposit.amount);
        }
    }

    #[endpoint(sendFromLiquidity)]
    fn send_from_liquidity(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        receiver: ManagedAddress,
    ) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();
        require!(self.relayer().get() == caller, ERR_NOT_PRIVILEGED);

        require!(
            self.liquidity(&token_identifier).get() >= amount,
            ERR_NOT_ENOUGH_LIQUIDITY
        );

        self.send_from_liquidity_event(
            &self.relayer().get(),
            &token_identifier,
            &amount,
            &receiver,
        );

        self.send()
            .direct_esdt(&receiver, &token_identifier, 0u64, &amount);

        self.liquidity(&token_identifier)
            .update(|value| *value -= amount);
    }
}
