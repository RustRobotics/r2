// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

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
    //void * data;
    pub data: *const u8,

    /// The event type that occurred.
    pub event_type: EventType,
}
