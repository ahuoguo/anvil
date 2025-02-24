use crate::kubernetes_cluster::spec::builtin_controllers::{garbage_collector::*, types::*};
use crate::state_machine::state_machine::*;
use vstd::prelude::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub open spec fn builtin_controllers() -> BuiltinControllersStateMachine {
    StateMachine {
        init: |s: ()| {
            true
        },
        actions: set![
            run_garbage_collector(),
        ],
        step_to_action: |step: BuiltinControllersStep| {
            match step {
                BuiltinControllersStep::RunGarbageCollector => run_garbage_collector(),
            }
        },
        action_input: |step: BuiltinControllersStep, input: BuiltinControllersActionInput| {
            input
        }
    }
}

}
