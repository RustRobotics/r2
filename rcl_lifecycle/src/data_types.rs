// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rcl::node::Node;
use rcl::publisher::Publisher;
use rcl::service::Service;

/// It contains the state of the lifecycle state machine
#[derive(Debug)]
pub struct LifecycleState {
    /// String with state name: Unconfigured, Inactive, Active or Finalized
    pub label: String,

    /// Identifier of the state
    pub id: u8,

    /// Pointer to a struct with the valid transitions
    pub valid_transitions: Box<LifecycleTransition>,

    /// Number of valid transitions
    pub valid_transition_size: u32,
}

/// It contains the transitions of the lifecycle state machine
#[derive(Debug)]
pub struct LifecycleTransition {
    /// String with transition name: configuring, cleaningup, activating, deactivating,
    /// errorprocessing or shuttingdown.
    pub label: String,

    /// Identifier of the transition
    pub id: u32,

    /// The value where the transition is initialized
    pub start: Box<LifecycleState>,
    /// The objetive of the transition
    pub goal: Box<LifecycleState>,
}

/// It contains the transition map states and transitions
#[derive(Debug)]
pub struct LifecycleTransitionMap {
    /// States used to generate the transition map
    pub states: Vec<LifecycleState>,

    /// Number of states
    pub states_size: u32,

    /// Transitions used to generate the transition map
    pub transitions: Vec<LifecycleTransition>,

    /// Number of transitions
    pub transitions_size: u32,
}

/// It contains the communication interfac with the R2 world
#[derive(Debug)]
pub struct LifecycleComInterface {
    /// Handle to the node used to create the publisher and the services
    pub node_handle: Box<Node>,

    /// Event used to publish the transitions
    pub pub_transition_event: Publisher,

    /// Service that allows to trigger changes on the state
    pub srv_change_state: Service,

    /// Service that allows to get the current state
    pub srv_get_state: Service,

    /// Service that allows to get the available states
    pub srv_get_available_states: Service,

    /// Service that allows to get the available transitions
    pub srv_get_available_transitions: Service,

    /// Service that allows to get transitions from the graph
    pub srv_get_transition_graph: Service,
}

/// It contains various options to configure the rcl_lifecycle_state_machine_t instance
#[derive(Debug)]
pub struct LifecycleStateMachineOptions {
    /// Flag indicating whether the state machine shall be initialized with default states
    pub initialize_default_states: bool,

    /// Flag indicating whether the com interface shall be used or not
    pub enable_com_interface: bool,
}

/// It contains the state machine data
#[derive(Debug)]
pub struct LifecycleStateMachine {
    /// Current state of the state machine
    pub current_state: Box<LifecycleState>,

    /// Map/Associated array of registered states and transitions
    pub transition_map: LifecycleTransitionMap,

    /// Communication interface into a R2 world
    pub com_interface: LifecycleComInterface,

    /// Options struct with which the state machine was initialized
    pub options: LifecycleStateMachineOptions,
}
