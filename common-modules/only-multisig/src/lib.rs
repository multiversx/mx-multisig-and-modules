#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OnlyMultisig {
    fn require_sc_address(&self, address: &ManagedAddress) {
        require!(
            !address.is_zero() && self.blockchain().is_smart_contract(address),
            "Invalid SC address"
        );
    }

    fn require_multisig_caller(&self) {
        let caller = self.blockchain().get_caller();
        let ms_address = self.multisig_address().get();
        require!(caller == ms_address, "Only multisig may call this endpoint");
    }

    #[view(getMultisigAddress)]
    #[storage_mapper("multisigAddress")]
    fn multisig_address(&self) -> SingleValueMapper<ManagedAddress>;
}
