use crate::storage::{InteractionStatus, PaymentsVec};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait MultisigInteractionsViews: crate::storage::MultisigInteractionsStorage {
    #[view(canExecute)]
    fn can_execute(
        &self,
        proposer: ManagedAddress,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
        egld_value: BigUint,
        esdt_payments: PaymentsVec<Self::Api>,
    ) -> bool {
        let sc_id = self.sc_address_to_id().get_id(&sc_address);
        if sc_id == NULL_ID {
            return false;
        }

        let allowed_users_mapper = self.allowed_users_for_interaction(sc_id, &endpoint_name);
        if !allowed_users_mapper.is_empty() && !allowed_users_mapper.contains(&proposer) {
            return false;
        }

        let opt_allowed_token = self.allowed_token(sc_id, &endpoint_name).get();
        match opt_allowed_token {
            Some(allowed_token) => {
                if egld_value > 0 && !allowed_token.is_egld() {
                    return false;
                }

                for payment in &esdt_payments {
                    if payment.token_identifier != allowed_token {
                        return false;
                    }
                }
            }
            None => {
                if egld_value > 0 || !esdt_payments.is_empty() {
                    return false;
                }
            }
        }

        true
    }

    /// A result of ManagedAddress::zero() means anyone is allowed to call this endpoint
    #[view(getAllowedUsersForInteraction)]
    fn get_allowed_users_for_interaction(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
    ) -> MultiValueEncoded<ManagedAddress> {
        let mut result = MultiValueEncoded::new();
        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);
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
        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);
        let opt_allowed_token = self.allowed_token(sc_id, &endpoint_name).get();
        match opt_allowed_token {
            Some(allowed_token) => OptionalValue::Some(allowed_token),
            None => OptionalValue::None,
        }
    }

    #[view(getInteractionStatus)]
    fn get_interaction_status(
        &self,
        sc_address: ManagedAddress,
        endpoint_name: ManagedBuffer,
    ) -> InteractionStatus {
        let sc_id = self.sc_address_to_id().get_id_non_zero(&sc_address);

        self.interaction_status(sc_id, &endpoint_name).get()
    }
}