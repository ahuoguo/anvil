// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::error::*;
use crate::kubernetes_api_objects::spec::{common::*, dynamic::*, object_meta::*, resource::*};
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

// ServiceAccountView is the ghost type of ServiceAccount.


pub struct ServiceAccountView {
    pub metadata: ObjectMetaView,
    pub automount_service_account_token: Option<bool>,
}

type ServiceAccountSpecView = Option<bool>;

impl ServiceAccountView {
    pub open spec fn set_metadata(self, metadata: ObjectMetaView) -> ServiceAccountView {
        ServiceAccountView {
            metadata: metadata,
            ..self
        }
    }
}

impl ResourceView for ServiceAccountView {
    type Spec = ServiceAccountSpecView;
    type Status = EmptyStatusView;

    open spec fn default() -> ServiceAccountView {
        ServiceAccountView {
            metadata: ObjectMetaView::default(),
            automount_service_account_token: None,
        }
    }

    open spec fn metadata(self) -> ObjectMetaView {
        self.metadata
    }

    open spec fn kind() -> Kind {
        Kind::ServiceAccountKind
    }

    open spec fn object_ref(self) -> ObjectRef {
        ObjectRef {
            kind: Self::kind(),
            name: self.metadata.name.get_Some_0(),
            namespace: self.metadata.namespace.get_Some_0(),
        }
    }

    proof fn object_ref_is_well_formed() {}

    open spec fn spec(self) -> ServiceAccountSpecView {
        self.automount_service_account_token
    }

    open spec fn status(self) -> EmptyStatusView {
        empty_status()
    }

    open spec fn marshal(self) -> DynamicObjectView {
        DynamicObjectView {
            kind: Self::kind(),
            metadata: self.metadata,
            spec: ServiceAccountView::marshal_spec(self.automount_service_account_token),
            status: ServiceAccountView::marshal_status(empty_status()),
        }
    }

    open spec fn unmarshal(obj: DynamicObjectView) -> Result<ServiceAccountView, UnmarshalError> {
            if obj.kind != Self::kind() {
                Err(())
            } else if !ServiceAccountView::unmarshal_spec(obj.spec).is_Ok() {
                Err(())
            } else if !ServiceAccountView::unmarshal_status(obj.status).is_Ok() {
                Err(())
            } else {
                Ok(ServiceAccountView {
                    metadata: obj.metadata,
                    automount_service_account_token: ServiceAccountView::unmarshal_spec(obj.spec).get_Ok_0(),
                })
            }
    }

    proof fn marshal_preserves_integrity() {
        ServiceAccountView::marshal_spec_preserves_integrity();
        ServiceAccountView::marshal_status_preserves_integrity();
    }

    proof fn marshal_preserves_metadata() {}

    proof fn marshal_preserves_kind() {}

    closed spec fn marshal_spec(s: ServiceAccountSpecView) -> Value;

    closed spec fn unmarshal_spec(v: Value) -> Result<ServiceAccountSpecView, UnmarshalError>;

    closed spec fn marshal_status(s: EmptyStatusView) -> Value;

    closed spec fn unmarshal_status(v: Value) -> Result<EmptyStatusView, UnmarshalError>;

    #[verifier(external_body)]
    proof fn marshal_spec_preserves_integrity() {}

    #[verifier(external_body)]
    proof fn marshal_status_preserves_integrity() {}

    proof fn unmarshal_result_determined_by_unmarshal_spec_and_status() {}

    open spec fn state_validation(self) -> bool {
        true
    }

    open spec fn transition_validation(self, old_obj: ServiceAccountView) -> bool {
        true
    }
}

}
