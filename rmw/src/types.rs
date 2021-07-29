// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::event::Event;
use super::init::Context;
use super::time::TimePointValue;

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

    /// Creates and publishes messages to the ROS topic
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

/// Options that can be used to configure the creation of a publisher in rmw.
#[derive(Debug)]
pub struct PublisherOptions {
    /// Used to pass rmw implementation specific resources during publisher creation.
    /**
     * This field is type erased (rather than forward declared) because it will
     * usually be a non-owned reference to an language specific object, e.g.
     * C++ it may be a polymorphic class that only the rmw implementation can use.
     *
     * The resource pointed to here needs to outlive this options structure, and
     * any rmw_publisher objects that are created using it, as they copy this
     * structure and may use this payload throughout their lifetime.
     */
    // FIXME(Shaohua):
    //void * rmw_specific_publisher_payload;
    pub rmw_specific_publisher_payload: *const u8,

    /// Require middleware to generate unique network flow endpoints.
    /**
     * Unique network flow endpoints are required to differentiate the QoS provided by
     * networks for flows between publishers and subscribers in communicating
     * nodes.
     * Default value is RMW_UNIQUE_NETWORK_FLOW_ENDPOINTS_NOT_REQUIRED.
     */
    pub require_unique_network_flow_endpoints: UniqueNetworkFlowEndpointsRequirement,
}

/// Structure which encapsulates an rmw publisher
#[derive(Debug)]
pub struct Publisher {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this publisher's data
    //void * data;
    pub data: *const u8,

    /// The name of the ROS topic this publisher publishes to
    pub topic_name: String,

    /// Publisher options.
    ///
    /// The options structure passed to rmw_create_publisher() should be
    /// assigned to this field by the rmw implementation.
    /// The fields should not be modified after creation, but
    /// the contents of the options structure may or may not be const, i.e.
    /// shallow const-ness.
    /// This field is not marked const to avoid any const casting during setup.
    pub options: PublisherOptions,

    /// Indicate whether this publisher supports loaning messages
    pub can_loan_messages: bool,
}

/// Options that can be used to configure the creation of a subscription in rmw.
#[derive(Debug)]
pub struct SubscriptionOptions {
    /// Used to pass rmw implementation specific resources during subscription creation.
    ///
    /// All the same details and restrictions of this field in
    /// `PublisherOptions` apply to this struct as well.
    /// rmw_publisher_options_t.rmw_specific_publisher_payload
    //void * rmw_specific_subscription_payload;
    pub rmw_specific_subscription_payload: *const u8,

    /// If true then the middleware should not deliver data from local publishers.
    ///
    /// This setting is most often used when data should only be received from
    /// remote nodes, especially to avoid "double delivery" when both intra- and
    /// inter- process communication is taking place.
    ///
    /// The definition of local is somewhat vague at the moment.
    /// Right now it means local to the node, and that definition works best, but
    /// may become more complicated when/if participants map to a context instead.
    pub ignore_local_publications: bool,

    /// Require middleware to generate unique network flow endpoints.
    ///
    /// Unique network flow endpoints are required to differentiate the QoS provided by
    /// networks for flows between publishers and subscribers in communicating
    /// nodes.
    /// Default value is RMW_UNIQUE_NETWORK_FLOW_ENDPOINTS_NOT_REQUIRED.
    pub require_unique_network_flow_endpoints: UniqueNetworkFlowEndpointsRequirement,
}

#[derive(Debug)]
pub struct Subscription {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this subscription
    //void * data;
    pub data: *const u8,

    /// Name of the ros topic this subscription listens to
    pub topic_name: String,

    /// Subscription options.
    ///
    /// The options structure passed to rmw_create_subscription() should be
    /// assigned to this field by the rmw implementation.
    /// The fields should not be modified after creation, but
    /// the contents of the options structure may or may not be const, i.e.
    /// shallow const-ness.
    /// This field is not marked const to avoid any const casting during setup.
    pub options: SubscriptionOptions,

    /// Indicates whether this subscription can loan messages
    pub can_loan_messages: bool,
}

/// A handle to an rmw service
#[derive(Debug)]
pub struct Service {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this service
    //void * data;
    pub data: *const u8,

    /// The name of this service as exposed to the ros graph
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

    /// The name of this service as exposed to the ros graph
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSReliabilityPolicy {
    /// Implementation specific default
    SystemDefault,

    /// Guarantee that samples are delivered, may retry multiple times.
    Reliable,

    /// Attempt to deliver samples, but some may be lost if the network is not robust
    BestEffort,

    /// Reliability policy has not yet been set
    Unknown,
}

/// QoS history enumerations describing how samples endure.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSHistoryPolicy {
    /// Implementation default for history policy
    SystemDefault = 0,

    /// Only store up to a maximum number of samples, dropping oldest once max is exceeded
    KeepLast,

    /// Store all samples, subject to resource limits
    KeepAll,

    /// History policy has not yet been set
    Unknown,
}

/// QoS durability enumerations describing how samples persist
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSDurabilityPolicy {
    /// Impplementation specific default
    SystemDefault = 0,

    /// The rmw publisher is responsible for persisting samples for “late-joining” subscribers
    TransientLocal,

    /// Samples are not persistent
    Volatile,

    /// Durability policy has not yet been set
    Unknown,
}

/// QoS liveliness enumerations that describe a publisher's reporting policy for its alive status.
/// For a subscriber, these are its requirements for its topic's publishers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSLivelinessPolicy {
    /// Implementation specific default
    SystemDefault = 0,

    /// The signal that establishes a Topic is alive comes from the ROS rmw layer.
    Automatic,

    /// Explicitly asserting node liveliness is required in this case.
    #[deprecated(since = "0.1", note = "Use `ManualByTopic` instead")]
    ManualByNode,

    /// The signal that establishes a Topic is alive is at the Topic level. Only publishing a message
    /// on the Topic or an explicit signal from the application to assert liveliness on the Topic
    /// will mark the Topic as being alive.
    ManualByTopic,

    /// Liveliness policy has not yet been set
    Unknown,
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
