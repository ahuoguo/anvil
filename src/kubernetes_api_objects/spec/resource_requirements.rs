// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub struct ResourceRequirementsView {
    pub limits: Option<Map<StringView, StringView>>,
    pub requests: Option<Map<StringView, StringView>>,
}

impl ResourceRequirementsView {
    pub open spec fn default() -> ResourceRequirementsView {
        ResourceRequirementsView {
            limits: None,
            requests: None,
        }
    }

    pub open spec fn set_limits(self, limits: Map<StringView, StringView>) -> ResourceRequirementsView {
        ResourceRequirementsView {
            limits: Some(limits),
            ..self
        }
    }

    pub open spec fn set_requests(self, requests: Map<StringView, StringView>) -> ResourceRequirementsView {
        ResourceRequirementsView {
            requests: Some(requests),
            ..self
        }
    }
}

}
