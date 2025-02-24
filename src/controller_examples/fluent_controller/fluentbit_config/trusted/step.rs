// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

#[is_variant]
pub enum FluentBitConfigReconcileStep {
    Init,
    AfterKRequestStep(ActionKind, SubResource),
    Done,
    Error,
}

impl std::marker::Copy for FluentBitConfigReconcileStep {}

impl std::clone::Clone for FluentBitConfigReconcileStep {
    #[verifier(external_body)]
    fn clone(&self) -> (result: Self)
        ensures result == self
    { *self }
}

pub enum SubResource {
    Secret,
}

impl std::marker::Copy for SubResource {}

impl std::clone::Clone for SubResource {
    #[verifier(external_body)]
    fn clone(&self) -> (result: Self)
        ensures result == self
    { *self }
}

#[is_variant]
pub enum ActionKind {
    Get,
    Create,
    Update,
}

impl std::marker::Copy for ActionKind {}

impl std::clone::Clone for ActionKind {
    #[verifier(external_body)]
    fn clone(&self) -> (result: Self)
        ensures result == self
    { *self }
}

}
