use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

#[is_variant]
pub enum VReplicaSetReconcileStep {
    Init,
    AfterListPods,
    AfterCreatePod(usize),
    AfterDeletePod(usize),
    Done,
    Error,
}

impl std::marker::Copy for VReplicaSetReconcileStep {}

impl std::clone::Clone for VReplicaSetReconcileStep {
    fn clone(&self) -> (result: Self)
        ensures result == self
    { *self }
}

impl View for VReplicaSetReconcileStep {
    type V = VReplicaSetRecStepView;

    open spec fn view(&self) -> VReplicaSetRecStepView {
        match self {
            VReplicaSetReconcileStep::Init => VReplicaSetRecStepView::Init,
            VReplicaSetReconcileStep::AfterListPods => VReplicaSetRecStepView::AfterListPods,
            VReplicaSetReconcileStep::AfterCreatePod(diff) => VReplicaSetRecStepView::AfterCreatePod(*diff as nat),
            VReplicaSetReconcileStep::AfterDeletePod(diff) => VReplicaSetRecStepView::AfterDeletePod(*diff as nat),
            VReplicaSetReconcileStep::Done => VReplicaSetRecStepView::Done,
            VReplicaSetReconcileStep::Error => VReplicaSetRecStepView::Error,
        }
    }
}

#[is_variant]
pub enum VReplicaSetRecStepView {
    Init,
    AfterListPods,
    AfterCreatePod(nat),
    AfterDeletePod(nat),
    Done,
    Error,
}

}
