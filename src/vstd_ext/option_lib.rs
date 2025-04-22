// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub open spec fn option_view<T: View>(o: Option<T>) -> Option<T::V> {
    match o {
        Some(t) => Some(t@),
        None => None,
    }
}

pub open spec fn option_vec_view<T: View>(o: Option<Vec<T>>) -> Option<Seq<T::V>> {
    match o {
        Some(vec) => Some(vec@.map_values(|t: T| t@)),
        None => None,
    }
}

}
