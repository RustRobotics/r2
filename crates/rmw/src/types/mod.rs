// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::time::TimePointValue;

mod client;
mod durability_policy;
mod gid;
mod guard_condition;
mod history_policy;
mod liveliness_policy;
mod node;
mod publisher;
mod publisher_options;
mod reliability_policy;
mod service;
mod subscription;
mod subscription_options;
mod wait_set;

pub use client::Client;
pub use durability_policy::QoSDurabilityPolicy;
pub use gid::{Gid, GID_STORAGE_SIZE};
pub use guard_condition::{GuardCondition, GuardConditions};
pub use history_policy::QoSHistoryPolicy;
pub use liveliness_policy::QoSLivelinessPolicy;
pub use node::{NodeBaseTrait, NodeTrait};
pub use publisher::{PublisherBaseTrait, PublisherTrait};
pub use publisher_options::PublisherOptions;
pub use reliability_policy::QoSReliabilityPolicy;
pub use service::Service;
pub use subscription::Subscription;
pub use subscription_options::SubscriptionOptions;
pub use wait_set::WaitSet;

/// Endpoint enumeration type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// Endpoint type has not yet been set
    Invalid = 0,

    /// Creates and publishes messages to the R2 topic
    Publisher,

    /// Listens for and receives messages from a topic
    Subscription,
}

/// Unique network flow endpoints requirement enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UniqueNetworkFlowEndpointsRequirement {
    /// Unique network flow endpoints not required
    NotRequired = 0,

    /// Unique network flow endpoins strictly required.
    /// Error if not provided by RMW implementation.
    StrictlyRequired,

    /// Unique network flow endpoints optionally required.
    /// No error if not provided RMW implementation.
    OptionallyRequired,

    /// Unique network flow endpoints requirement decided by system.
    SystemDefault,
}

/// Array of subscriber handles.
///
/// An array of void * pointers representing type-erased middleware-specific subscriptions.
/// The number of non-null entries may be smaller than the allocated size of the array.
/// The number of subscriptions represented may be smaller than the allocated size of the array.
/// The creator of this struct is responsible for allocating and deallocating the array.
#[derive(Debug)]
pub struct Subscriptions(Vec<Subscription>);

/// Array of service handles.
///
/// An array of void * pointers representing type-erased middleware-specific services.
/// The number of non-null entries may be smaller than the allocated size of the array.
/// The number of services represented may be smaller than the allocated size of the array.
/// The creator of this struct is responsible for allocating and deallocating the array.
#[derive(Debug)]
pub struct Services(Vec<Service>);

/// Array of client handles.
///
/// An array of void * pointers representing type-erased middleware-specific clients.
/// The number of non-null entries may be smaller than the allocated size of the array.
/// The number of clients represented may be smaller than the allocated size of the array.
/// The creator of this struct is responsible for allocating and deallocating the array.
#[derive(Debug)]
pub struct Clients(Vec<Client>);

/// An rmw service request identifier
#[derive(Debug)]
pub struct RequestId {
    /// The guid of the writer associated with this request
    pub writer_guid: [i8; 16],

    /// Sequence number of this service
    pub sequence_number: i64,
}

/// Meta-data for a service-related take.
#[derive(Debug)]
pub struct ServiceInfo {
    pub source_timestamp: TimePointValue,
    pub received_timestamp: TimePointValue,
    pub request_id: RequestId,
}

/// Information describing an rmw message
#[derive(Debug)]
pub struct MessageInfo {
    pub source_timestamp: TimePointValue,
    pub received_timestamp: TimePointValue,
    pub publisher_gid: Gid,

    /// Whether this message is from intra_process communication or not
    pub from_intra_process: bool,
}
