use super::storage::{Interaction, DISABLED, ENABLED};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait MultisigInteractions:
    only_multisig::OnlyMultisig + super::storage::MultisigInteractionsStorage
{
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
        self.require_sc_address(&sc_address);
        self.require_interaction_not_added(sc_address.clone(), endpoint_name.clone());

        if let Some(token_id) = &opt_allowed_token_id {
            require!(token_id.is_valid(), "Invalid token ID");
        }

        let sc_id = self.sc_address_to_id().get_id_or_insert(&sc_address);
        self.add_allowed_users_for_interaction(sc_id, &endpoint_name, allowed_addresses);
        self.allowed_token(sc_id, &endpoint_name)
            .set(opt_allowed_token_id);
        self.interaction_status(sc_id, &endpoint_name).set(ENABLED);
        self.all_interactions().insert(Interaction {
            sc_address,
            endpoint_name,
        });
    }

    #[endpoint(disableInteraction)]
    fn disable_interaction(&self, sc_address: ManagedAddress, endpoint_name: ManagedBuffer) {
        self.require_multisig_caller();

        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);
        self.interaction_status(sc_id, &endpoint_name).set(DISABLED);
    }

    #[endpoint(enableInteraction)]
    fn enable_interaction(&self, sc_address: ManagedAddress, endpoint_name: ManagedBuffer) {
        self.require_multisig_caller();

        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);
        self.interaction_status(sc_id, &endpoint_name).set(ENABLED);
    }

    #[endpoint(addAllowedAddresses)]
    fn add_allowed_addresses(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
        allowed_addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_multisig_caller();

        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);
        self.add_allowed_users_for_interaction(sc_id, &endpoint_name, allowed_addresses);
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

    fn add_allowed_users_for_interaction(
        &self,
        sc_id: AddressId,
        endpoint_name: &ManagedBuffer,
        allowed_addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        let mut interaction_user_mapper = self.allowed_users_for_interaction(sc_id, endpoint_name);
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
}
