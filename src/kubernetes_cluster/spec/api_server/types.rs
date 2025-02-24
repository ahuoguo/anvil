// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::external_api::spec::*;
use crate::kubernetes_api_objects::spec::{api_method::*, common::*, dynamic::*};
use crate::kubernetes_cluster::spec::message::*;
use crate::state_machine::action::*;
use crate::state_machine::state_machine::*;

use crate::temporal_logic::defs::*;
use vstd::{multiset::*, prelude::*};

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub struct ApiServerState {
    pub resources: StoredState,
    pub uid_counter: Uid,
    pub resource_version_counter: ResourceVersion,
    pub stable_resources: Set<ObjectRef>,
}

pub enum ApiServerStep {
    HandleRequest,
}

pub struct ApiServerActionInput<I, O> {
    pub recv: Option<Message<I, O>>,
}

pub struct ApiServerActionOutput<I, O> {
    pub send: Multiset<Message<I, O>>
}

pub type ApiServerStateMachine<I, O> = StateMachine<ApiServerState, ApiServerActionInput<I, O>, ApiServerActionInput<I, O>, ApiServerActionOutput<I, O>, ApiServerStep>;

pub type ApiServerAction<I, O> = Action<ApiServerState, ApiServerActionInput<I, O>, ApiServerActionOutput<I, O>>;

}
