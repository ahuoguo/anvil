// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::exec::{object_meta::*, pod::*, resource::*};
use crate::kubernetes_api_objects::spec::pod_template_spec::*;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

#[verifier(external_body)]
pub struct PodTemplateSpec {
    inner: deps_hack::k8s_openapi::api::core::v1::PodTemplateSpec,
}

impl PodTemplateSpec {
    pub spec fn view(&self) -> PodTemplateSpecView;

    #[verifier(external_body)]
    pub fn default() -> (pod_template_spec: PodTemplateSpec)
        ensures pod_template_spec@ == PodTemplateSpecView::default(),
    {
        PodTemplateSpec {
            inner: deps_hack::k8s_openapi::api::core::v1::PodTemplateSpec::default(),
        }
    }

    #[verifier(external_body)]
    pub fn clone(&self) -> (pod_template_spec: PodTemplateSpec)
        ensures pod_template_spec@ == self@,
    {
        PodTemplateSpec { inner: self.inner.clone() }
    }

    #[verifier(external_body)]
    pub fn metadata(&self) -> (metadata: Option<ObjectMeta>)
        ensures
            self@.metadata.is_Some() == metadata.is_Some(),
            metadata.is_Some() ==> metadata.get_Some_0()@ == self@.metadata.get_Some_0(),
    {
        match &self.inner.metadata {
            Some(m) => Some(ObjectMeta::from_kube(m.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn spec(&self) -> (spec: Option<PodSpec>)
        ensures
            self@.spec.is_Some() == spec.is_Some(),
            spec.is_Some() ==> spec.get_Some_0()@ == self@.spec.get_Some_0(),
    {
        match &self.inner.spec {
            Some(s) => Some(PodSpec::from_kube(s.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn set_metadata(&mut self, metadata: ObjectMeta)
        ensures self@ == old(self)@.set_metadata(metadata@),
    {
        self.inner.metadata = Some(metadata.into_kube());
    }

    #[verifier(external_body)]
    pub fn set_spec(&mut self, spec: PodSpec)
        ensures self@ == old(self)@.set_spec(spec@),
    {
        self.inner.spec = Some(spec.into_kube());
    }

    #[verifier(external)]
    pub fn from_kube(inner: deps_hack::k8s_openapi::api::core::v1::PodTemplateSpec) -> PodTemplateSpec {
        PodTemplateSpec { inner: inner }
    }

    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::core::v1::PodTemplateSpec { self.inner }
}

}
