// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub struct VolumeResourceRequirementsView {
    pub limits: Option<Map<StringView, StringView>>,
    pub requests: Option<Map<StringView, StringView>>,
}

impl VolumeResourceRequirementsView {
    pub open spec fn default() -> VolumeResourceRequirementsView {
        VolumeResourceRequirementsView {
            limits: None,
            requests: None,
        }
    }

    pub open spec fn set_limits(self, limits: Map<StringView, StringView>) -> VolumeResourceRequirementsView {
        VolumeResourceRequirementsView {
            limits: Some(limits),
            ..self
        }
    }

    pub open spec fn set_requests(self, requests: Map<StringView, StringView>) -> VolumeResourceRequirementsView {
        VolumeResourceRequirementsView {
            requests: Some(requests),
            ..self
        }
    }
}

}
