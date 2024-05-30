multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getLiquidity)]
    #[storage_mapper("liquidity")]
    fn liquidity(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getTokenDecimals)]
    #[storage_mapper("token_decimals")]
    fn token_decimals(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<u32>;

    #[view(getMinimumDeposit)]
    #[storage_mapper("minimum_deposit")]
    fn minimum_deposit(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getMaximumDeposit)]
    #[storage_mapper("maximum_deposit")]
    fn maximum_deposit(&self, token_identifier: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getFeeCollector)]
    #[storage_mapper("fee_colector")]
    fn fee_colector(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getFeeValue)]
    #[storage_mapper("fee_value")]
    fn fee_value(&self) -> SingleValueMapper<BigUint>;

    #[view(getWegldContractAddress)]
    #[storage_mapper("wegld_contract_address")]
    fn wegld_contract_address(&self) -> SingleValueMapper<ManagedAddress>;
}
