multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UtilsModule {
    fn check_amount(&self, amount: &BigUint, decimals: u32) -> bool {
        let token_decimals = BigUint::from(10u32).pow(decimals);

        amount % &token_decimals == 0
    }
}
