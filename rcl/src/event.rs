// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

/// Enumeration of all of the publisher events that may fire.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PublisherEventType {
    OfferedDeadlineMissed,
    LivelinessLost,
    OfferedIncompatibleQoS,
}

/// Enumeration of all of the subscription events that may fire.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum SubscriptionEventType {
    RequestedDeadlineMissed,
    LivelinessChanged,
    RequestedIncompatibleQoS,
    MessageLost,
}

/// Internal rcl implementation struct.
pub trait EventImpl: Debug {}

/// Structure which encapsulates a R2 QoS event handle.
#[derive(Debug)]
pub struct Event {
    /// Pointer to the event implementation
    pub imp: Box<dyn EventImpl>,
}
