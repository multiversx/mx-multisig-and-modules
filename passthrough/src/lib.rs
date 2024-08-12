#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Passthrough:
    only_by_multisig::OnlyByMultisig + multisig_interactions::MultisigInteractions
{
    #[init]
    fn init(&self, multisig_address: ManagedAddress) {
        self.require_sc_address(&multisig_address);

        self.multisig_address().set(multisig_address);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
