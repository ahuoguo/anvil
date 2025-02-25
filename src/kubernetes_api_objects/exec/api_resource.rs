// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::exec::resource::*;
use crate::kubernetes_api_objects::spec::api_resource::*;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

// ApiResource is used for creating API handles for DynamicObject.
//
// This definition is a wrapper of ApiResource defined at
// https://github.com/kube-rs/kube/blob/main/kube-core/src/discovery.rs.
// It is supposed to be used in exec controller code.

#[verifier(external_body)]
pub struct ApiResource {
    inner: deps_hack::kube::api::ApiResource,
}

impl ApiResource {
    pub spec fn view(&self) -> ApiResourceView;

    #[verifier(external)]
    pub fn as_kube_ref(&self) -> &deps_hack::kube::api::ApiResource {
        &self.inner
    }
}

#[verifier(external)]
impl ResourceWrapper<deps_hack::kube::api::ApiResource> for ApiResource {
    fn from_kube(inner: deps_hack::kube::api::ApiResource) -> ApiResource {
        ApiResource {
            inner: inner
        }
    }

    fn into_kube(self) -> deps_hack::kube::api::ApiResource {
        self.inner
    }
}

}
