// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::exec::resource::*;
use crate::kubernetes_api_objects::spec::toleration::*;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

// The pod this Toleration is attached to tolerates any taint that matches
//
// This definition is a wrapper of Toleration defined at
// https://github.com/Arnavion/k8s-openapi/blob/v0.17.0/src/v1_26/api/core/v1/toleration.rs.
// It is supposed to be used in exec controller code.
//
// More detailed information: https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/.

#[verifier(external_body)]
pub struct Toleration {
    inner: deps_hack::k8s_openapi::api::core::v1::Toleration,
}

impl Toleration {
    pub spec fn view(&self) -> TolerationView;
}

#[verifier(external)]
impl ResourceWrapper<deps_hack::k8s_openapi::api::core::v1::Toleration> for Toleration {
    fn from_kube(inner: deps_hack::k8s_openapi::api::core::v1::Toleration) -> Toleration { Toleration { inner: inner } }

    fn into_kube(self) -> deps_hack::k8s_openapi::api::core::v1::Toleration { self.inner }
}

}
