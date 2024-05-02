#![no_std]

multiversx_sc::imports!();

pub mod config;
pub mod errors;
pub mod storage;

#[multiversx_sc::contract]
pub trait CoreMxBridgeSc: storage::StorageModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
