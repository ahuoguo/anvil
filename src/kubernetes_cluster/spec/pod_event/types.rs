// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::kubernetes_api_objects::spec::{pod::*, resource::*};
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

pub type PodEventState = ();

pub enum Step {
    CreatePod(PodView),
    UpdatePod(PodView),
    UpdatePodStatus(PodView),
    DeletePod(PodView),
}

pub struct PodEventActionInput {
    pub pod: PodView,
    pub rest_id_allocator: RestIdAllocator,
}

pub struct PodEventActionOutput<I, O> {
    pub send: Multiset<Message<I, O>>,
    pub rest_id_allocator: RestIdAllocator,
}

pub type PodEventStateMachine<I, O> = StateMachine<PodEventState, RestIdAllocator, PodEventActionInput, PodEventActionOutput<I, O>, Step>;

pub type PodEventAction<I, O> = Action<PodEventState, PodEventActionInput, PodEventActionOutput<I, O>>;

}
