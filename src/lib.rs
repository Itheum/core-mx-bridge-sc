#![no_std]

use crate::errors::{
    ERR_CONTRACT_NOT_READY, ERR_NOT_ENOUGH_LIQUIDITY, ERR_NOT_PRIVILEGED, ERR_TOKEN_NOT_WHITELISTED,
};

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod errors;
pub mod macros;
pub mod storage;

#[multiversx_sc::contract]
pub trait CoreMxBridgeSc:
    storage::StorageModule + config::ConfigModule + admin::AdminModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(sendToLiquidity)]
    fn send_to_liquidity(&self) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let payment = self.call_value().single_esdt();

        require!(
            self.tokens_whitelist().contains(&payment.token_identifier),
            ERR_TOKEN_NOT_WHITELISTED
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

        self.send()
            .direct_esdt(&address, &token_identifier, 0u64, &amount);

        self.liquidity(&token_identifier)
            .update(|value| *value -= amount);
    }
}
