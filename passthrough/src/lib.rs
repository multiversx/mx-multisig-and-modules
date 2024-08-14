#![no_std]

multiversx_sc::imports!();

pub mod multisig_interactions;

#[multiversx_sc::contract]
pub trait Passthrough:
    only_multisig::OnlyMultisig
    + multisig_interactions::main::MultisigInteractions
    + multisig_interactions::views::MultisigInteractionsViews
    + multisig_interactions::storage::MultisigInteractionsStorage
{
    #[init]
    fn init(&self, multisig_address: ManagedAddress) {
        self.require_sc_address(&multisig_address);

        self.multisig_address().set(multisig_address);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
