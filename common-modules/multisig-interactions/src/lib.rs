#![no_std]

multiversx_sc::imports!();

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;

#[multiversx_sc::module]
pub trait MultisigInteractions: only_by_multisig::OnlyByMultisig {
    /// If allowed_addresses is empty, any account can call this endpoint
    #[endpoint(addInteraction)]
    fn add_interaction(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
        allowed_addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_multisig_caller();
    }

    #[endpoint(disableInteraction)]
    fn disable_interaction(&self, sc_address: ManagedAddress, endpoint_name: ManagedBuffer) {
        self.require_multisig_caller();
    }

    #[endpoint(enableInteraction)]
    fn enable_interaction(&self, sc_address: ManagedAddress, endpoint_name: ManagedBuffer) {
        self.require_multisig_caller();
    }

    #[view(canExecute)]
    fn can_execute(
        &self,
        proposer: ManagedAddress,
        sc_address: ManagedAddress,
        egld_value: BigUint,
        esdt_payments: PaymentsVec<Self::Api>,
    ) -> bool {
        false
    }
}
