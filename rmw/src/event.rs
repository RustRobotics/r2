// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::ret_types::RetType;
use crate::types::{PublisherTrait, Subscription};

/// Define publisher/subscription events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    /// Subscription events
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
pub trait EventBaseTrait: Clone {
    /// Implementation identifier, used to ensure two different implementations are not being mixed.
    fn implementation_identifier(&self) -> &'static str;

    /// Data specific to this event type from either the publisher or subscriber.
    fn data(&self) -> &[u8];

    /// The event type that occurred.
    fn event_type(&self) -> EventType;

    /// Return a zero initialized event structure.
    fn zero_initialized() -> Self;
}

/// Initialize a rmw subscription event.
pub trait EventTrait: EventBaseTrait {
    /// Initialize a rmw publisher event.
    fn publisher_event_init(
        &mut self,
        publisher: &dyn PublisherTrait,
        event_type: EventType,
    ) -> RetType;

    /// Initialize a rmw subscription event.
    fn subscription_event_init(
        &mut self,
        subscription: &Subscription,
        event_type: EventType,
    ) -> RetType;

    /// Take an event from the event handle.
    fn take_event(&mut self, event_info: &mut usize, taken: &mut bool) -> RetType;
}

#[derive(Debug)]
pub struct Events(Vec<dyn EventTrait>);
