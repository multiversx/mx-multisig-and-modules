#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub const ENABLED: bool = true;
pub const DISABLED: bool = false;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Interaction<M: ManagedTypeApi> {
    pub sc_address: ManagedAddress<M>,
    pub endpoint_name: ManagedBuffer<M>,
}

#[multiversx_sc::module]
pub trait MultisigInteractions: only_by_multisig::OnlyByMultisig {
    /// If allowed_addresses is empty, any account can call this endpoint
    /// For EGLD as allowed token, simply pass Some("EGLD")
    #[endpoint(addInteraction)]
    fn add_interaction(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
        opt_allowed_token_id: Option<EgldOrEsdtTokenIdentifier>,
        allowed_addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_multisig_caller();
        self.require_interaction_not_added(sc_address.clone(), endpoint_name.clone());

        if let Some(token_id) = &opt_allowed_token_id {
            require!(token_id.is_valid(), "Invalid token ID");
        }

        let sc_id = self.sc_address_to_id().get_id_or_insert(&sc_address);
        self.add_allowed_users_for_interaction(sc_id, &endpoint_name, allowed_addresses);
        self.allowed_token(sc_id, &endpoint_name)
            .set(opt_allowed_token_id);
        self.all_interactions().insert(Interaction {
            sc_address,
            endpoint_name,
        });
    }

    #[endpoint(disableInteraction)]
    fn disable_interaction(&self, sc_address: ManagedAddress, endpoint_name: ManagedBuffer) {
        self.require_multisig_caller();
    }

    #[endpoint(enableInteraction)]
    fn enable_interaction(&self, sc_address: ManagedAddress, endpoint_name: ManagedBuffer) {
        self.require_multisig_caller();
    }

    #[endpoint(addAllowedAddresses)]
    fn add_allowed_addresses(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
        allowed_addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_multisig_caller();
    }

    #[endpoint(setAllowedTokenForInteraction)]
    fn set_allowed_token_for_interaction(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
        opt_allowed_token_id: Option<EgldOrEsdtTokenIdentifier>,
    ) {
        self.require_multisig_caller();

        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);
        self.allowed_token(sc_id, &endpoint_name)
            .set(opt_allowed_token_id);
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

    /// A result of ManagedAddress::zero() means anyone is allowed to call this endpoint
    #[view(getAllowedUsersForInteraction)]
    fn get_allowed_users_for_interaction(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
    ) -> MultiValueEncoded<ManagedAddress> {
        let mut result = MultiValueEncoded::new();
        let sc_id = self.sc_address_to_id().get_id(&sc_address);
        if sc_id == NULL_ID {
            return result;
        }

        let mapper = self.allowed_users_for_interaction(sc_id, &endpoint_name);
        if !mapper.is_empty() {
            for user in mapper.iter() {
                result.push(user);
            }
        } else {
            result.push(ManagedAddress::zero())
        }

        result
    }

    #[view(getAllowedTokenForInteraction)]
    fn get_allowed_token_for_interaction(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
    ) -> OptionalValue<EgldOrEsdtTokenIdentifier> {
        let sc_id = self.sc_address_to_id().get_id(&sc_address);
        if sc_id == NULL_ID {
            return OptionalValue::None;
        }

        let opt_allowed_token = self.allowed_token(sc_id, &endpoint_name).get();
        match opt_allowed_token {
            Some(allowed_token) => OptionalValue::Some(allowed_token),
            None => OptionalValue::None,
        }
    }

    fn add_allowed_users_for_interaction(
        &self,
        sc_id: AddressId,
        endpoint_name: &ManagedBuffer,
        allowed_addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        let mut interaction_user_mapper = self.allowed_users_for_interaction(sc_id, &endpoint_name);
        for user in allowed_addresses {
            interaction_user_mapper.insert(user);
        }
    }

    fn require_interaction_not_added(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
    ) {
        let sc_id = self.sc_address_to_id().get_id(&sc_address);
        if sc_id == NULL_ID {
            return;
        }

        let interaction = Interaction {
            sc_address,
            endpoint_name,
        };
        require!(
            !self.all_interactions().contains(&interaction),
            "Interaction already added"
        );
    }

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

    #[storage_mapper("tok")]
    fn allowed_token(
        &self,
        sc_id: AddressId,
        endpoint_name: &ManagedBuffer,
    ) -> SingleValueMapper<Option<EgldOrEsdtTokenIdentifier>>;
}
