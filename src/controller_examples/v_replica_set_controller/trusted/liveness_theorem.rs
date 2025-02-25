// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::v_replica_set_controller::trusted::{spec_types::*, step::*};
use crate::kubernetes_api_objects::spec::prelude::*;
use crate::kubernetes_cluster::spec::{cluster::*, cluster_state_machine::Step, message::*};
use crate::temporal_logic::defs::*;
use crate::vstd_ext::string_view::int_to_string_view;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub open spec fn liveness_theorem() -> bool { cluster_spec().entails(tla_forall(|vrs: VReplicaSetView| liveness(vrs))) }

pub open spec fn cluster_spec() -> TempPred<VRSCluster> { VRSCluster::sm_spec() }

pub open spec fn liveness(vrs: VReplicaSetView) -> TempPred<VRSCluster> {
    always(lift_state(desired_state_is(vrs))).leads_to(always(lift_state(current_state_matches(vrs))))
}

pub open spec fn desired_state_is(vrs: VReplicaSetView) -> StatePred<VRSCluster> { VRSCluster::desired_state_is(vrs) }

pub open spec fn current_state_matches(vrs: VReplicaSetView) -> StatePred<VRSCluster> {
    |s: VRSCluster| {
        resource_state_matches(vrs, s.resources())
    }
}

pub open spec fn resource_state_matches(vrs: VReplicaSetView, resources: StoredState) -> bool {
    let pods: Set<ObjectRef> = Set::new(|key: ObjectRef| {
        let obj = resources[key];
        &&& resources.contains_key(key)
        &&& owned_selector_match_is(vrs, obj)
    });
    &&& pods.len() == vrs.spec.replicas.unwrap_or(0)
}

pub open spec fn owned_selector_match_is(vrs: VReplicaSetView, obj: DynamicObjectView) -> bool {
    &&& obj.kind == PodView::kind()
    &&& obj.metadata.namespace.is_Some()
    &&& obj.metadata.namespace == vrs.metadata.namespace
    &&& obj.metadata.owner_references_contains(vrs.controller_owner_ref())
    &&& vrs.spec.selector.matches(obj.metadata.labels.unwrap_or(Map::empty()))
    &&& obj.metadata.deletion_timestamp.is_None()
}

}
