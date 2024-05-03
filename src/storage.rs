multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getLiquidity)]
    #[storage_mapper("liquidity")]
    fn liquidity(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getMinimumDeposit)]
    #[storage_mapper("minimum_deposit")]
    fn minimum_deposit(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getMaximumDeposit)]
    #[storage_mapper("maximum_deposit")]
    fn maximum_deposit(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<BigUint>;
}
