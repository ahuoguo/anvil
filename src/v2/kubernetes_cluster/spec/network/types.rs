use crate::kubernetes_cluster::spec::message::*;
use vstd::{multiset::*, prelude::*};

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub struct NetworkState {
    pub in_flight: Multiset<Message>,
}

}
