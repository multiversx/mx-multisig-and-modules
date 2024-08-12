use adder::Adder;
use multisig_improved::{
    common_types::{
        action::{ActionId, Nonce},
        signature::{ActionType, SignatureArg, SignatureType},
    },
    ms_endpoints::{
        perform::PerformEndpointsModule, propose::ProposeEndpointsModule, sign::SignEndpointsModule,
    },
    Multisig,
};
use multiversx_sc::{
    codec::{Empty, TopEncode},
    imports::OptionalValue,
    types::{Address, FunctionCall, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    imports::{BlockchainStateWrapper, ContractObjWrapper},
    managed_address, managed_biguint, managed_buffer, rust_biguint, DebugApi,
};
use passthrough::Passthrough;

pub struct PassSetup<PassThroughBuilder, MsImprovedBuilder, AdderBuilder>
where
    PassThroughBuilder: 'static + Copy + Fn() -> passthrough::ContractObj<DebugApi>,
    MsImprovedBuilder: 'static + Copy + Fn() -> multisig_improved::ContractObj<DebugApi>,
    AdderBuilder: 'static + Copy + Fn() -> adder::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub first_board_member: Address,
    pub second_board_member: Address,
    pub ms_owner: Address,
    pub pass_wrapper: ContractObjWrapper<passthrough::ContractObj<DebugApi>, PassThroughBuilder>,
    pub ms_wrapper: ContractObjWrapper<multisig_improved::ContractObj<DebugApi>, MsImprovedBuilder>,
    pub adder_wrapper: ContractObjWrapper<adder::ContractObj<DebugApi>, AdderBuilder>,
}

impl<PassThroughBuilder, MsImprovedBuilder, AdderBuilder>
    PassSetup<PassThroughBuilder, MsImprovedBuilder, AdderBuilder>
where
    PassThroughBuilder: 'static + Copy + Fn() -> passthrough::ContractObj<DebugApi>,
    MsImprovedBuilder: 'static + Copy + Fn() -> multisig_improved::ContractObj<DebugApi>,
    AdderBuilder: 'static + Copy + Fn() -> adder::ContractObj<DebugApi>,
{
    pub fn new(
        pass_builder: PassThroughBuilder,
        ms_builder: MsImprovedBuilder,
        adder_builder: AdderBuilder,
    ) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let first_board_member = b_mock.create_user_account(&rust_zero);
        let second_board_member = b_mock.create_user_account(&rust_zero);
        let ms_owner = b_mock.create_user_account(&rust_zero);
        let adder_wrapper = b_mock.create_sc_account(
            &rust_zero,
            Some(&first_board_member),
            adder_builder,
            "adder",
        );
        let ms_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&ms_owner), ms_builder, "multisig");
        let pass_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&ms_owner), pass_builder, "passthrough");

        // init adder
        b_mock
            .execute_tx(&first_board_member, &adder_wrapper, &rust_zero, |sc| {
                sc.init(managed_biguint!(0));
            })
            .assert_ok();

        // init multisig
        b_mock
            .execute_tx(&ms_owner, &ms_wrapper, &rust_zero, |sc| {
                let mut board = MultiValueEncoded::new();
                board.push(managed_address!(&first_board_member));
                board.push(managed_address!(&second_board_member));

                sc.init(2, board);
            })
            .assert_ok();

        // init passthrough
        let ms_address = ms_wrapper.address_ref().clone();
        b_mock
            .execute_tx(&ms_owner, &pass_wrapper, &rust_zero, |sc| {
                sc.init(managed_address!(&ms_address));
            })
            .assert_ok();

        Self {
            b_mock,
            first_board_member,
            second_board_member,
            pass_wrapper,
            ms_owner,
            ms_wrapper,
            adder_wrapper,
        }
    }

    pub fn propose_transfer_execute(
        &mut self,
        to: &Address,
        egld_amount: u64,
        function_name: &[u8],
        args: Vec<Vec<u8>>,
    ) -> ActionId {
        let mut action_id = 0;

        self.b_mock
            .execute_tx(
                &self.first_board_member,
                &self.ms_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let mut function_call = FunctionCall::new(function_name);
                    for arg in args {
                        function_call = function_call.argument(&arg);
                    }

                    action_id = sc
                        .propose_transfer_execute(
                            managed_address!(to),
                            managed_biguint!(egld_amount),
                            None,
                            function_call,
                            OptionalValue::None,
                        )
                        .into_option()
                        .unwrap();
                },
            )
            .assert_ok();

        action_id
    }

    pub fn perform(&mut self, action_id: ActionId) {
        self.b_mock
            .execute_tx(
                &self.first_board_member,
                &self.ms_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let _ = sc.perform_action_endpoint(action_id);
                },
            )
            .assert_ok();
    }

    pub fn perform_and_expect_err(&mut self, action_id: ActionId, err_message: &str) {
        self.b_mock
            .execute_tx(
                &self.first_board_member,
                &self.ms_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let _ = sc.perform_action_endpoint(action_id);
                },
            )
            .assert_user_error(err_message);
    }

    pub fn sign(&mut self, action_id: ActionId, signer_nonce: Nonce) {
        let signer_addr = self.second_board_member.clone();

        self.b_mock
            .execute_tx(
                &self.second_board_member,
                &self.ms_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let mut signatures = MultiValueEncoded::new();
                    signatures.push(SignatureArg {
                        user_address: managed_address!(&signer_addr),
                        nonce: signer_nonce,
                        action_type: ActionType::SimpleAction,
                        raw_sig_bytes: managed_buffer!(b"signature"),
                        signature_type: SignatureType::Ed25519, // unused
                    });

                    sc.sign(action_id, signatures);
                },
            )
            .assert_ok();
    }

    pub fn propose_add_module(&mut self, sc_address: &Address) -> ActionId {
        let mut action_id = 0;

        self.b_mock
            .execute_tx(
                &self.first_board_member,
                &self.ms_wrapper,
                &rust_biguint!(0),
                |sc| {
                    action_id =
                        sc.propose_add_module(managed_address!(sc_address), OptionalValue::None);
                },
            )
            .assert_ok();

        action_id
    }

    pub fn propose_add_interaction(
        &mut self,
        sc_address: &Address,
        endpoint_name: &[u8],
        allowed_addresses: Vec<Address>,
    ) -> ActionId {
        let mut args = Vec::new();
        let mut encoded_sc_address = Vec::new();
        let _ = sc_address.top_encode(&mut encoded_sc_address);
        args.push(encoded_sc_address);
        args.push(endpoint_name.to_vec());

        let mut encoded_opt = Vec::new();
        let _ = Option::<Empty>::None.top_encode(&mut encoded_opt);
        args.push(encoded_opt);

        for address in allowed_addresses {
            let mut encoded_address = Vec::new();
            let _ = address.top_encode(&mut encoded_address);
            args.push(encoded_address);
        }

        self.propose_transfer_execute(
            &self.pass_wrapper.address_ref().clone(),
            0,
            b"addInteraction",
            args,
        )
    }
}
