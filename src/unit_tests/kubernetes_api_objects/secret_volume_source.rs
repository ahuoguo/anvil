// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::container::*;
use crate::kubernetes_api_objects::object_meta::*;
use crate::kubernetes_api_objects::pod::*;
use crate::kubernetes_api_objects::resource::*;
use crate::kubernetes_api_objects::volume::*;
use crate::vstd_ext::string_map::*;
use vstd::prelude::*;
use vstd::string::*;

verus! {
// Tests for secret volume source
#[test]
#[verifier(external)]
pub fn test_default() {
    let secret_volume_source = SecretVolumeSource::default();
    assert_eq!(secret_volume_source.into_kube(), deps_hack::k8s_openapi::api::core::v1::SecretVolumeSource::default());
}

#[test]
#[verifier(external)]
pub fn test_set_secret_name() {
    let mut secret_volume_source = SecretVolumeSource::default();
    secret_volume_source.set_secret_name(new_strlit("secret_name").to_string());
    assert_eq!("secret_name".to_string(), secret_volume_source.into_kube().secret_name.unwrap());
}

}
