multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getLiquidity)]
    #[storage_mapper("liquidity")]
    fn liquidity(&self) -> SingleValueMapper<BigUint>;
}
