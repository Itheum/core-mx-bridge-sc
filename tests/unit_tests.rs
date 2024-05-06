use core_mx_bridge_sc::utils::UtilsModule;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::SingleTxApi;

mod bridge_sc;

#[test]
fn test_bridge_sc() {
    let bridge_sc = core_mx_bridge_sc::contract_obj::<SingleTxApi>();

    let mut check = bridge_sc.check_amount(&BigUint::from(10u64).pow(17), 18u32); // 0.1 tokens

    assert_eq!(check, false);

    check = bridge_sc.check_amount(&(BigUint::from(5u64) * BigUint::from(10u64).pow(17)), 18u32); // 0.5 tokens

    assert_eq!(check, false);

    check = bridge_sc.check_amount(&(BigUint::from(1u64) * BigUint::from(10u64).pow(18)), 18u32); // 1 token

    assert_eq!(check, true);

    check = bridge_sc.check_amount(
        &(BigUint::from(20u64) * BigUint::from(10u64).pow(18)),
        18u32,
    ); // 20 tokens

    assert_eq!(check, true);

    check = bridge_sc.check_amount(&BigUint::from(2111111111111111111u64), 18u32); // 2.1111 tokens

    assert_eq!(check, false);
}
