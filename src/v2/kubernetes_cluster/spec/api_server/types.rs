use crate::kubernetes_api_objects::spec::prelude::*;
use crate::kubernetes_cluster::spec::message::*;
use crate::state_machine::action::*;
use crate::state_machine::state_machine::*;
use crate::vstd_ext::string_view::StringView;
use vstd::{multiset::*, prelude::*};

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub struct APIServerState {
    pub resources: StoredState,
    pub uid_counter: Uid,
    pub resource_version_counter: ResourceVersion,
}

pub type InstalledTypes = Map<StringView, InstalledType>;

pub struct InstalledType {
    pub unmarshallable_spec: spec_fn(Value) -> bool,
    pub unmarshallable_status: spec_fn(Value) -> bool,
    pub valid_object: spec_fn(DynamicObjectView) -> bool,
    pub valid_transition: spec_fn(DynamicObjectView, DynamicObjectView) -> bool,
    pub marshalled_default_status: spec_fn() -> Value,
}

pub enum APIServerStep {
    HandleRequest,
}

pub struct APIServerActionInput {
    pub recv: Option<Message>,
}

pub struct APIServerActionOutput {
    pub send: Multiset<Message>
}

pub type APIServerStateMachine = StateMachine<APIServerState, APIServerActionInput, APIServerActionInput, APIServerActionOutput, APIServerStep>;

pub type APIServerAction = Action<APIServerState, APIServerActionInput, APIServerActionOutput>;

}
