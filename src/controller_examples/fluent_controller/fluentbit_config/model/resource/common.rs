// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::fluent_controller::fluentbit_config::trusted::{spec_types::*, step::*};
use crate::kubernetes_api_objects::spec::{
    container::*, label_selector::*, pod_template_spec::*, prelude::*, resource_requirements::*,
    volume::*,
};
use crate::reconciler::spec::{io::*, reconciler::*};
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;
use vstd::string::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub open spec fn make_owner_references(fbc: FluentBitConfigView) -> Seq<OwnerReferenceView> {
    seq![fbc.controller_owner_ref()]
}

}
