// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT

#![allow(unused_imports)]

#![verifier::exec_allows_no_decreases_clause]
pub mod executable_model;
pub mod external_api;
pub mod kubernetes_api_objects;
pub mod kubernetes_cluster;
pub mod reconciler;
pub mod shim_layer;
pub mod state_machine;
pub mod temporal_logic;
pub mod vstd_ext;

use vstd::prelude::*;
