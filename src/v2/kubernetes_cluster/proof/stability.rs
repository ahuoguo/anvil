use crate::kubernetes_cluster::spec::{cluster::*, message::Message};
use crate::kubernetes_api_objects::spec::prelude::ObjectRef;
use crate::state_machine::action::*;
use crate::temporal_logic::{defs::*, rules::*};
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

impl Cluster {

// Prove weak_fairness is stable.
pub proof fn action_weak_fairness_is_stable<Output>(action: Action<ClusterState, (), Output>)
    ensures
        valid(stable(action.weak_fairness(()))),
{
    let split_always = always(lift_state(action.pre(()))).implies(eventually(lift_action(action.forward(()))));
    always_p_is_stable::<ClusterState>(split_always);
}

// Prove weak_fairness for all input is stable.
pub proof fn tla_forall_action_weak_fairness_is_stable<Input, Output>(
    action: Action<ClusterState, Input, Output>
)
    ensures
        valid(stable(tla_forall(|input| action.weak_fairness(input)))),
{
    let split_always = |input| always(lift_state(action.pre(input))).implies(eventually(lift_action(action.forward(input))));
    tla_forall_always_equality_variant::<ClusterState, Input>(|input| action.weak_fairness(input), split_always);
    always_p_is_stable::<ClusterState>(tla_forall(split_always));
}

// Prove weak_fairness for controller_next is stable.
pub proof fn tla_forall_controller_next_weak_fairness_is_stable(
    self, controller_id: int
)
    ensures
        valid(stable(tla_forall(|i: (Option<Message>, Option<ObjectRef>)| self.controller_next().weak_fairness((controller_id, i.0, i.1))))),
{
    let split_always = |i: (Option<Message>, Option<ObjectRef>)| {
        always(lift_state(self.controller_next().pre((controller_id, i.0, i.1))))
            .implies(eventually(lift_action(self.controller_next().forward((controller_id, i.0, i.1)))))
    };
    tla_forall_always_equality_variant(|i: (Option<Message>, Option<ObjectRef>)| self.controller_next().weak_fairness((controller_id, i.0, i.1)), split_always);
    always_p_is_stable(tla_forall(split_always));
}

}

}
