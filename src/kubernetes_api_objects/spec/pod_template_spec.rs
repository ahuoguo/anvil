// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::spec::{object_meta::*, pod::*};
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub struct PodTemplateSpecView {
    pub metadata: Option<ObjectMetaView>,
    pub spec: Option<PodSpecView>,
}

impl PodTemplateSpecView {
    pub open spec fn default() -> PodTemplateSpecView {
        PodTemplateSpecView {
            metadata: None,
            spec: None,
        }
    }

    pub open spec fn set_metadata(self, metadata: ObjectMetaView) -> PodTemplateSpecView {
        PodTemplateSpecView {
            metadata: Some(metadata),
            ..self
        }
    }

    pub open spec fn set_spec(self, spec: PodSpecView) -> PodTemplateSpecView {
        PodTemplateSpecView {
            spec: Some(spec),
            ..self
        }
    }
}

}
