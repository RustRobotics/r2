// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

use super::{Clients, Events, GuardConditions, Services, Subscriptions};
use crate::ret_types::RetType;

/// Container for guard conditions to be waited on
#[derive(Debug)]
pub struct WaitSet {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// The guard condition to be waited on
    pub guard_conditions: Box<GuardConditions>,

    /// Type erased pointer to this wait set's data
    //void * data;
    pub data: *const u8,
}

pub trait WaitSetTrait {
    /// Destroy a wait set.
    fn destroy(wait_set: &mut WaitSet) -> RetType;

    /// Waits on sets of different entities and returns when one is ready.
    fn wait(
        subscriptions: &mut Subscriptions,
        guard_conditions: GuardConditions,
        services: &mut Services,
        clients: &mut Clients,
        events: &mut Events,
        wait_set: &mut WaitSet,
        wait_timeout: &Duration,
    ) -> RetType;
}
