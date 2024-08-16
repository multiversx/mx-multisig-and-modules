use adder::Adder;
use multiversx_sc_scenario::rust_biguint;
use pass_setup::PassSetup;

pub mod pass_setup;

#[test]
fn setup_test() {
    let _ = PassSetup::new(
        passthrough::contract_obj,
        multisig_improved::contract_obj,
        adder::contract_obj,
    );
}

#[test]
fn add_module_test() {
    let mut setup = PassSetup::new(
        passthrough::contract_obj,
        multisig_improved::contract_obj,
        adder::contract_obj,
    );

    let action_id = setup.propose_add_module(&setup.pass_wrapper.address_ref().clone());
    setup.sign(action_id, 0);
    setup.perform(action_id);
}

#[test]
fn add_interaction_test() {
    let mut setup = PassSetup::new(
        passthrough::contract_obj,
        multisig_improved::contract_obj,
        adder::contract_obj,
    );

    let action_id = setup.propose_add_module(&setup.pass_wrapper.address_ref().clone());
    setup.sign(action_id, 0);
    setup.perform(action_id);

    let nice_guy = setup.b_mock.create_user_account(&rust_biguint!(0));
    let action_id = setup.propose_add_interaction(
        &setup.adder_wrapper.address_ref().clone(),
        b"add",
        vec![nice_guy],
    );
    setup.sign(action_id, 1);
    setup.perform(action_id);
}

#[test]
fn execute_without_signatures_test() {
    let mut setup = PassSetup::new(
        passthrough::contract_obj,
        multisig_improved::contract_obj,
        adder::contract_obj,
    );

    let action_id = setup.propose_add_module(&setup.pass_wrapper.address_ref().clone());
    setup.sign(action_id, 0);
    setup.perform(action_id);

    let nice_guy = setup.b_mock.create_user_account(&rust_biguint!(0));
    let action_id = setup.propose_add_interaction(
        &setup.adder_wrapper.address_ref().clone(),
        b"add",
        vec![nice_guy.clone()],
    );
    setup.sign(action_id, 1);
    setup.perform(action_id);

    let args = [[5u8][..].to_vec()].to_vec();
    setup.propose_transfer_execute_no_sig(
        &nice_guy,
        &setup.adder_wrapper.address_ref().clone(),
        0,
        b"add",
        args,
    );

    setup
        .b_mock
        .execute_query(&setup.adder_wrapper, |sc| {
            assert_eq!(sc.sum().get(), 5);
        })
        .assert_ok();
}

#[test]
fn try_execute_action_not_whitelisted() {
    let mut setup = PassSetup::new(
        passthrough::contract_obj,
        multisig_improved::contract_obj,
        adder::contract_obj,
    );

    let action_id = setup.propose_add_module(&setup.pass_wrapper.address_ref().clone());
    setup.sign(action_id, 0);
    setup.perform(action_id);

    let nice_guy = setup.b_mock.create_user_account(&rust_biguint!(0));
    let action_id = setup.propose_add_interaction(
        &setup.adder_wrapper.address_ref().clone(),
        b"add",
        vec![nice_guy.clone()],
    );
    setup.sign(action_id, 1);
    setup.perform(action_id);

    let evil_guy = setup.b_mock.create_user_account(&rust_biguint!(0));
    let args = [[5u8][..].to_vec()].to_vec();
    setup.propose_transfer_execute_no_sig_expect_err(
        &evil_guy,
        &setup.adder_wrapper.address_ref().clone(),
        0,
        b"add",
        args,
        "only board members and proposers can propose",
    );
}
