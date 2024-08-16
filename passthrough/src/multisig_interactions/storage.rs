multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type InteractionStatus = bool;
pub const ENABLED: InteractionStatus = true;
pub const DISABLED: InteractionStatus = false;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Interaction<M: ManagedTypeApi> {
    pub sc_address: ManagedAddress<M>,
    pub endpoint_name: ManagedBuffer<M>,
}

#[multiversx_sc::module]
pub trait MultisigInteractionsStorage {
    #[storage_mapper("scAddressToId")]
    fn sc_address_to_id(&self) -> AddressToIdMapper<Self::Api>;

    #[storage_mapper("allInteractions")]
    fn all_interactions(&self) -> UnorderedSetMapper<Interaction<Self::Api>>;

    #[storage_mapper("inter")]
    fn allowed_users_for_interaction(
        &self,
        sc_id: AddressId,
        endpoint_name: &ManagedBuffer,
    ) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("token")]
    fn allowed_token(
        &self,
        sc_id: AddressId,
        endpoint_name: &ManagedBuffer,
    ) -> SingleValueMapper<Option<EgldOrEsdtTokenIdentifier>>;

    #[storage_mapper("status")]
    fn interaction_status(
        &self,
        sc_id: AddressId,
        endpoint_name: &ManagedBuffer,
    ) -> SingleValueMapper<InteractionStatus>;
}
