// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::kubernetes_api_objects::spec::prelude::*;
use crate::reconciler::spec::{io::*, reconciler::*};
use crate::temporal_logic::defs::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;
use vstd::string::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub trait ResourceBuilder<K, T> {
    spec fn get_request(cr: K) -> GetRequest;

    spec fn make(cr: K, state: T) -> Result<DynamicObjectView, ()>;

    spec fn update(cr: K, state: T, obj: DynamicObjectView) -> Result<DynamicObjectView, ()>;

    spec fn state_after_create(cr: K, obj: DynamicObjectView, state: T) -> Result<(T, Option<APIRequest>), ()>;

    spec fn state_after_update(cr: K, obj: DynamicObjectView, state: T) -> Result<(T, Option<APIRequest>), ()>;
}

}
