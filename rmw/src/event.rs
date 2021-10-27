// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::ret_types::RetType;
use crate::types::{Publisher, Subscription};

/// Define publisher/subscription events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    /// subscription events
    LivelinessChanged,
    RequestedDeadlineMissed,
    RequestedQoSIncompatible,
    MessageLost,

    /// publisher events
    LivelinessLost,
    OfferedDeadlineMissed,
    OfferedQoSIncompatible,

    /// sentinel value
    Invalid,
}

/// Encapsulate the RMW event implementation, data, and type.
#[derive(Debug, Clone)]
pub struct Event {
    /// Implementation identifier, used to ensure two different implementations are not being mixed.
    pub implementation_identifier: String,

    /// Data specific to this event type from either the publisher or subscriber.
    //TODO(Shaohua):
    //void * data;
    pub data: usize,

    /// The event type that occurred.
    pub event_type: EventType,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            implementation_identifier: "".to_string(),
            data: 0,
            event_type: EventType::Invalid,
        }
    }
}

impl Event {
    /// Return a zero initialized event structure.
    pub fn zero_initialized() -> Self {
        Self::default()
    }
}

/// Initialize a rmw subscription event.
pub trait EventTrait {
    /// Initialize a rmw publisher event.
    //TODO(Shaohua): Replace with boxed pointer.
    fn publisher_event_init(
        event: &mut Event,
        publisher: &Publisher,
        event_type: EventType,
    ) -> RetType;

    /// Initialize a rmw subscription event.
    fn subscription_event_init(
        event: &mut Event,
        subscription: &Subscription,
        event_type: EventType,
    ) -> RetType;

    /// Take an event from the event handle.
    fn take_event(event: &mut Event, event_info: &mut usize, taken: &mut bool) -> RetType;
}
