// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::event::Event;
use crate::init::Context;
use crate::time::TimePointValue;

mod durability_policy;
mod history_policy;
mod liveliness_policy;
mod publisher;
mod publisher_options;
mod reliability_policy;
mod subscription;
mod subscription_options;

pub use durability_policy::QoSDurabilityPolicy;
pub use history_policy::QoSHistoryPolicy;
pub use liveliness_policy::QoSLivelinessPolicy;
pub use publisher::Publisher;
pub use publisher_options::PublisherOptions;
pub use reliability_policy::QoSReliabilityPolicy;
pub use subscription::Subscription;
pub use subscription_options::SubscriptionOptions;

/// 24 bytes is the most memory needed to represent the GID by any current
/// implementation. It may need to be increased in the future.
pub const GID_STORAGE_SIZE: usize = 24;

/// Structure which encapsulates an rmw node
#[derive(Debug)]
pub struct Node {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this node's data
    //void * data;

    /// A concise name of this rmw node for identification
    pub name: String,

    /// The namespace of this rmw node
    pub namespace_: String,

    /// Context information about node's init specific information
    pub context: Box<Context>,
}

/// Endpoint enumeration type
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
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
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
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

/// A handle to an rmw service
#[derive(Debug)]
pub struct Service {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this service
    //void * data;
    pub data: *const u8,

    /// The name of this service as exposed to the r2 graph
    pub service_name: String,
}

/// A handle to an rmw service client
#[derive(Debug)]
pub struct Client {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this service client
    //void * data;
    pub data: *const u8,

    /// The name of this service as exposed to the r2 graph
    pub service_name: String,
}

/// Handle for an rmw guard condition
#[derive(Debug)]
pub struct GuardCondition {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this guard condition
    //void * data;
    pub data: *const u8,

    /// rmw context associated with this guard condition
    pub context: Box<Context>,
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

#[derive(Debug)]
pub struct Events(Vec<Event>);

/// Array of guard condition handles.
///
/// An array of void * pointers representing type-erased middleware-specific guard conditions.
/// The number of non-null entries may be smaller than the allocated size of the array.
/// The number of guard conditions represented may be smaller than the allocated size of the array.
/// The creator of this struct is responsible for allocating and deallocating the array.
#[derive(Debug)]
pub struct GuardConditions(Vec<GuardCondition>);

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

/// R2 graph ID of the topic.
#[derive(Debug)]
pub struct Gid {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Byte data Gid value
    pub data: [u8; GID_STORAGE_SIZE],
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
