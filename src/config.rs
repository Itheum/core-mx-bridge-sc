multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{errors::ERR_ALREADY_IN_STORAGE, events, storage};

#[derive(
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Copy,
    ManagedVecItem,
)]
pub enum State {
    Inactive,
    Active,
}

#[multiversx_sc::module]
pub trait ConfigModule: storage::StorageModule + events::EventsModule {
    #[only_owner]
    #[endpoint(setAdministrator)]
    fn set_administrator(&self, administrator: ManagedAddress) {
        self.set_administrator_event(&administrator);

        if !self.administrator().is_empty() {
            require!(
                administrator != self.administrator().get(),
                ERR_ALREADY_IN_STORAGE
            );
        }
        self.administrator().set(administrator);
    }

    #[inline]
    fn is_whitelisted(&self, address: &ManagedAddress) -> bool {
        self.whitelist().contains(address)
    }

    #[inline]
    fn is_whitelist_active(&self) -> bool {
        self.is_state_active(self.whitelist_state().get())
    }

    #[inline]
    fn is_contract_owner(&self, address: &ManagedAddress) -> bool {
        &(self.blockchain().get_owner_address()) == address
    }

    #[inline]
    fn is_admin(&self, address: &ManagedAddress) -> bool {
        &(self.administrator().get()) == address
    }

    #[inline]
    fn is_relayer(&self, address: &ManagedAddress) -> bool {
        &(self.relayer().get()) == address
    }

    #[inline]
    fn is_privileged(&self, address: &ManagedAddress) -> bool {
        self.is_contract_owner(address) || self.is_admin(address)
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    fn contract_is_ready(&self) -> bool {
        let mut is_ready = true;

        if !self.is_state_active(self.contract_state().get()) {
            is_ready = false;
        }

        if self.administrator().is_empty() {
            is_ready = false;
        }

        if self.fee_collector().is_empty() {
            is_ready = false;
        }

        if self.wegld_contract_address().is_empty() {
            is_ready = false;
        }

        if self.relayer().is_empty() {
            is_ready = false;
        }
        if self.tokens_whitelist().is_empty() {
            is_ready = false;
        }
        is_ready
    }

    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTokensWhitelist)]
    #[storage_mapper("tokens_whitelist")]
    fn tokens_whitelist(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[storage_mapper("whitelist")]
    fn whitelist(&self) -> WhitelistMapper<ManagedAddress>;

    #[view(getWhitelistState)]
    #[storage_mapper("whitelist_state")]
    fn whitelist_state(&self) -> SingleValueMapper<State>;

    #[view(getContractState)]
    #[storage_mapper("contract_state")]
    fn contract_state(&self) -> SingleValueMapper<State>;

    #[view(getRelayer)]
    #[storage_mapper("relayer")]
    fn relayer(&self) -> SingleValueMapper<ManagedAddress>;
}
