use core_mx_bridge_sc::config::ProxyTrait as _;
use core_mx_bridge_sc::ProxyTrait as _;
use core_mx_bridge_sc::{admin::ProxyTrait as _, config::State};
use multiversx_sc::imports::SingleValue;
use multiversx_sc::{
    imports::MultiValue2,
    types::{Address, BigUint, MultiValueEncoded},
};
use multiversx_sc_scenario::imports::*;
use multiversx_sc_scenario::scenario_model::BigUintValue;

pub const BRIDGE_CONTRACT_PATH: &str = "mxsc:output/core-mx-bridge-sc-mxsc.json";

pub const WEGLD_SWAP_CONTRACT_PATH: &str =
    "mxsc:tests-contracts/multiversx-wegld-swap-sc.mxsc.json";

pub const BRIDGE_CONTRACT_ADDRESS_EXPR: &str = "sc:bridge-sc";

pub const WEGLD_SWAP_CONTRACT_ADDRESS_EXPR: &str = "sc:wegld-swap-sc";

pub const OWNER_WEGLD_SWAP_CONTRACT_ADDRESS_EXPR: &str = "address:owner-wegld-swap-sc";

pub const OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR: &str = "address:owner-bridge-sc";

pub const ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR: &str = "address:admin-bridge-sc";

pub const RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR: &str = "address:relayer-bridge-sc";

pub const WEGLD_TOKEN_IDENTIFIER_EXPR: &str = "str:WEGLD-fce905";
pub const WEGLD_TOKEN_IDENTIFIER: &[u8] = b"WEGLD-fce905";

pub const ITHEUM_TOKEN_IDENTIFIER_EXPR: &str = "str:ITHEUM-fce905";
pub const ITHEUM_TOKEN_IDENTIFIER: &[u8] = b"ITHEUM-fce905";

pub const ANOTHER_TOKEN_IDENTIFIER_EXPR: &str = "str:ANOTHER-fce905";
pub const ANOTHER_TOKEN_IDENTIFIER: &[u8] = b"ANOTHER-fce905";

pub const FIRST_USER_ADDRESS_EXPR: &str = "address:first_user";
pub const SECOND_USER_ADDRESS_EXPR: &str = "address:second_user";

pub const THIRD_USER_ADDRESS_EXPR: &str = "address:third_user";

type Contract = ContractInfo<core_mx_bridge_sc::Proxy<StaticApi>>;
type WegldSwapContract = ContractInfo<multiversx_wegld_swap_sc::Proxy<StaticApi>>;

pub fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract(BRIDGE_CONTRACT_PATH, core_mx_bridge_sc::ContractBuilder);

    blockchain.register_contract(
        WEGLD_SWAP_CONTRACT_PATH,
        multiversx_wegld_swap_sc::ContractBuilder,
    );

    blockchain
}

pub struct ContractState {
    pub world: ScenarioWorld,
    pub contract: Contract,
    pub wegld_swap_contract: WegldSwapContract,
    pub contract_owner: Address,
    pub admin: Address,
    pub relayer: Address,
    pub first_user: Address,
    pub second_user: Address,
    pub third_user: Address,
}

impl ContractState {
    pub fn new() -> Self {
        let mut world = world();

        world.set_state_step(
            SetStateStep::new()
                .put_account(
                    OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "5_000_000"),
                )
                .new_address(
                    OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                    1,
                    BRIDGE_CONTRACT_ADDRESS_EXPR,
                )
                .put_account(
                    OWNER_WEGLD_SWAP_CONTRACT_ADDRESS_EXPR,
                    Account::new().nonce(1).balance("1_000"),
                )
                .new_address(
                    OWNER_WEGLD_SWAP_CONTRACT_ADDRESS_EXPR,
                    1,
                    WEGLD_SWAP_CONTRACT_ADDRESS_EXPR,
                )
                .put_account(
                    ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000_000")
                        .esdt_balance(ANOTHER_TOKEN_IDENTIFIER_EXPR, "10_000_000"),
                )
                .put_account(
                    RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                    Account::new().nonce(1),
                )
                .put_account(
                    FIRST_USER_ADDRESS_EXPR,
                    Account::new()
                        .balance("100")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000")
                        .esdt_balance(ANOTHER_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .put_account(
                    SECOND_USER_ADDRESS_EXPR,
                    Account::new()
                        .balance("100")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000")
                        .esdt_balance(ANOTHER_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .put_account(THIRD_USER_ADDRESS_EXPR, Account::new().balance("100")),
        );

        let contract = Contract::new(BRIDGE_CONTRACT_ADDRESS_EXPR);
        let wegld_swap_contract = WegldSwapContract::new(WEGLD_SWAP_CONTRACT_ADDRESS_EXPR);

        let contract_owner = AddressValue::from(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR).to_address();
        let admin = AddressValue::from(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR).to_address();
        let relayer = AddressValue::from(RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR).to_address();
        let first_user = AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address();
        let second_user = AddressValue::from(SECOND_USER_ADDRESS_EXPR).to_address();
        let third_user = AddressValue::from(THIRD_USER_ADDRESS_EXPR).to_address();

        Self {
            world,
            contract,
            wegld_swap_contract,
            contract_owner,
            admin,
            relayer,
            first_user,
            second_user,
            third_user,
        }
    }

    pub fn default_deploy_and_set(&mut self) -> &mut Self {
        self.deploy()
            .add_token_to_whitelist(
                OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                ITHEUM_TOKEN_IDENTIFIER,
                None,
            )
            .set_administrator(
                OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                AddressValue::from(ADMIN_BRIDGE_CONTRACT_ADDRESS_EXPR).to_address(),
                None,
            )
            .set_relayer(
                OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                AddressValue::from(RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR).to_address(),
                None,
            )
            .set_fee_collector(
                OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                AddressValue::from(RELAYER_BRIDGE_CONTRACT_ADDRESS_EXPR).to_address(),
                None,
            )
            .set_wegld_contract_address(
                OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR,
                AddressValue::from(WEGLD_SWAP_CONTRACT_ADDRESS_EXPR).to_address(),
                None,
            );

        self
    }

    pub fn deploy_wegld_swap(&mut self) -> &mut Self {
        let wegld_swap_contract_code = self.world.code_expression(WEGLD_SWAP_CONTRACT_PATH);

        let mut acc = Account::new()
            .esdt_roles(
                WEGLD_TOKEN_IDENTIFIER_EXPR,
                vec![
                    "ESDTRoleLocalBurn".to_string(),
                    "ESDTRoleLocalMint".to_string(),
                ],
            )
            .code(wegld_swap_contract_code);

        acc.storage.insert(
            b"wrappedEgldTokenId".to_vec().into(),
            WEGLD_TOKEN_IDENTIFIER.to_vec().into(),
        );

        acc.storage.insert(
            b"pause_module:paused".to_vec().into(),
            "false".to_string().into(),
        );

        acc.owner = Option::Some(AddressValue::from(OWNER_WEGLD_SWAP_CONTRACT_ADDRESS_EXPR));
        self.world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(WEGLD_TOKEN_IDENTIFIER_EXPR)
                .put_account(WEGLD_SWAP_CONTRACT_ADDRESS_EXPR, acc),
        );

        self
    }

    pub fn deploy(&mut self) -> &mut Self {
        let bridge_contract_code = self.world.code_expression(BRIDGE_CONTRACT_PATH);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_BRIDGE_CONTRACT_ADDRESS_EXPR)
                .code(bridge_contract_code)
                .call(self.contract.init()),
        );
        self
    }

    pub fn check_contract_state(&mut self, contract_state: State) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.contract_state())
                .expect_value(SingleValue::from(contract_state)),
        );
        self
    }

    pub fn set_administrator(
        &mut self,
        caller: &str,
        administrator: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_administrator(administrator))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_relayer(
        &mut self,
        caller: &str,
        relayer: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_relayer(relayer))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_fee_value(
        &mut self,
        caller: &str,
        amount: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_fee_value(BigUint::from(amount)))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_contract_state_active(
        &mut self,
        caller: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_contract_state_active())
                .expect(tx_expect),
        );
        self
    }

    pub fn set_contract_state_inactive(
        &mut self,
        caller: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_contract_state_inactive())
                .expect(tx_expect),
        );
        self
    }

    pub fn set_whitelist_state_active(
        &mut self,
        caller: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_whitelist_state_active())
                .expect(tx_expect),
        );
        self
    }

    pub fn set_whitelist_state_inactive(
        &mut self,
        caller: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_whitelist_state_inactive())
                .expect(tx_expect),
        );
        self
    }

    pub fn add_to_whitelist(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        let mut addresses = MultiValueEncoded::new();
        addresses.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.add_to_whitelist(addresses))
                .expect(tx_expect),
        );
        self
    }

    pub fn remove_from_whitelist(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        let mut addresses = MultiValueEncoded::new();
        addresses.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_from_whitelist(addresses))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_deposit_limits(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        min_deposit: &[u8],
        max_deposit: &[u8],
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_deposit_limits(
                    managed_token_id!(token_identifier),
                    BigUint::from(managed_buffer!(min_deposit)),
                    BigUint::from(managed_buffer!(max_deposit)),
                ))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_fee_collector(
        &mut self,
        caller: &str,
        fee_collector: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_fee_collector(fee_collector))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_wegld_contract_address(
        &mut self,
        caller: &str,
        wegld_contract_address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .set_wegld_contract_address(wegld_contract_address),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn add_token_to_whitelist(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        let mut args = MultiValueEncoded::new();
        args.push(MultiValue2((managed_token_id!(token_identifier), 18u32)));
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.add_tokens_to_whitelist(args))
                .expect(tx_expect),
        );
        self
    }

    pub fn remove_token_from_whitelist(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        let mut args = MultiValueEncoded::new();
        args.push(managed_token_id!(token_identifier));
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_tokens_from_whitelist(args))
                .expect(tx_expect),
        );
        self
    }

    pub fn add_to_liquidity(
        &mut self,
        caller: &str,
        payment: (&str, u64, u64),
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(payment.0, payment.1, BigUintValue::from(payment.2))
                .call(self.contract.add_to_liquidity())
                .expect(tx_expect),
        );
        self
    }

    pub fn remove_from_liquidity(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        amount: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_from_liquidity(
                    managed_token_id!(token_identifier),
                    BigUint::from(amount),
                ))
                .expect(tx_expect),
        );
        self
    }

    pub fn send_to_liquidity(
        &mut self,
        caller: &str,
        payment: (&str, u64, &str),
        extra_arguments: Vec<&[u8]>,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(payment.0, payment.1, BigUintValue::from(payment.2))
                .call(
                    self.contract
                        .send_to_liquidity(extra_arguments[0], extra_arguments[1]),
                )
                .expect(tx_expect),
        );

        self
    }

    pub fn send_to_liquidity_with_fee(
        &mut self,
        caller: &str,
        payment: (&str, u64, &str),
        fee: (&str, u64, &str),
        extra_arguments: Vec<&[u8]>,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        // let mut tokens = Vec::<TxESDT>::new();

        // tokens.push(TxESDT {
        //     esdt_token_identifier: BytesValue::from(payment.0),
        //     nonce: U64Value::from(payment.1),
        //     esdt_value: BigUintValue::from(payment.2),
        // });

        // tokens.push(TxESDT {
        //     esdt_token_identifier: BytesValue::from(fee.0),
        //     nonce: U64Value::from(fee.1),
        //     esdt_value: BigUintValue::from(fee.2),
        // });

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(payment.0, payment.1, BigUintValue::from(payment.2))
                .esdt_transfer(fee.0, fee.1, BigUintValue::from(fee.2))
                .call(
                    self.contract
                        .send_to_liquidity(extra_arguments[0], extra_arguments[1]),
                )
                .expect(tx_expect),
        );

        self
    }

    pub fn send_from_liquidity(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        amount: u64,
        address: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.send_from_liquidity(
                    managed_token_id!(token_identifier),
                    BigUint::from(amount),
                    AddressValue::from(address).to_address(),
                ))
                .expect(tx_expect),
        );

        self
    }
}
