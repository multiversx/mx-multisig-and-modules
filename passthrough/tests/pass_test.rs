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
