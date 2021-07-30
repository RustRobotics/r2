// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use super::client::Client;
use super::event::Event;
use super::guard_condition::GuardCondition;
use super::service::Service;
use super::subscription::Subscription;
use super::timer::Timer;

pub trait WaitSetImpl: Debug {}

/// Container for subscription's, guard condition's, etc to be waited on.
#[derive(Debug)]
pub struct WaitSet {
    /// Storage for subscription pointers.
    pub subscriptions: Box<Vec<Subscription>>,

    /// Number of subscriptions
    // TODO(Shaohua): Remove this field.
    pub size_of_subscriptions: usize,

    /// Storage for guard condition pointers.
    pub guard_conditions: Box<Vec<GuardCondition>>,

    /// Number of guard_conditions
    pub size_of_guard_conditions: usize,

    /// Storage for timer pointers.
    pub timers: Box<Vec<Timer>>,

    /// Number of timers
    pub size_of_timers: usize,

    /// Storage for client pointers.
    pub clients: Box<Vec<Client>>,

    /// Number of clients
    pub size_of_clients: usize,

    /// Storage for service pointers.
    pub services: Box<Vec<Service>>,

    /// Number of services
    pub size_of_services: usize,

    /// Storage for event pointers.
    pub events: Box<Vec<Event>>,

    /// Number of events
    pub size_of_events: usize,

    /// Implementation specific storage.
    pub imp: Box<dyn WaitSetImpl>,
}
