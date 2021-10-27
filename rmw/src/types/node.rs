// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::Context;
use crate::names_and_types::NamesAndTypes;
use crate::qos_profiles::QoSProfile;
use crate::ret_types::RetType;
use crate::topic_endpoint_info_array::TopicEndpointInfoArray;
use crate::types::{
    GuardCondition, Publisher, PublisherOptions, Subscription, SubscriptionOptions,
};

/// Structure which encapsulates an rmw node
#[derive(Debug)]
pub struct Node {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this node's data.
    // TODO(Shaohua):
    //void * data;
    pub data: usize,

    /// A concise name of this rmw node for identification.
    pub name: String,

    /// The namespace of this rmw node.
    pub namespace_: String,

    /// Context information about node's init specific information.
    pub context: Option<Box<Context>>,
}

pub trait NodeTrait {
    /// Return all topic names and types for which a given remote node has subscriptions.
    ///
    /// This function returns an array of topic names and types for which a given remote
    /// node has subscriptions, as discovered so far by the given local node.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying subscribed topic names and types:
    /// - Access to the array of names and types is not synchronized.
    ///   It is not safe to read or write `topic_names_and_types`
    ///   while get_subscriber_names_and_types() uses it.
    /// - Access to node name and namespace is read-only but it is not synchronized.
    ///   Concurrent `node_name` and `node_namespace` reads are safe, but concurrent reads and writes are not.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    ///
    /// Given `topic_names_and_types` must be a zero-initialized array of names and types,
    /// as returned by get_zero_initialized_names_and_types().
    ///
    /// Return `RET_OK` if the query was successful, or return `RET_INVALID_ARGUMENT` if `node` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `node_name` is not valid, by validate_node_name() definition,
    /// or return `RET_INVALID_ARGUMENT` if `node_namespace` is not valid, by validate_namespace() definition,
    /// or return `RET_INVALID_ARGUMENT` if `topic_names_and_types` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_NODE_NAME_NON_EXISTENT` if the node name wasn't found,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_subscriber_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all topic names and types for which a given remote node has publishers.
    ///
    /// This function returns an array of topic names and types for which a given remote
    /// node has created publishers, as discovered so far by the given local node.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying published topic names and types:
    /// - Access to the array of names and types is not synchronized.
    ///   It is not safe to read or write `topic_names_and_types`
    ///   while get_publisher_names_and_types() uses it.
    /// - Access to node name and namespace is read-only but it is not synchronized.
    ///   Concurrent `node_name` and `node_namespace` reads are safe, but concurrent reads and
    ///   writes are not.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    ///
    /// Given `topic_names_and_types` must be a zero-initialized array of names and types,
    /// as returned by get_zero_initialized_names_and_types().
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `node` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `node_name` is not valid, by validate_node_name() definition,
    /// or return `RET_INVALID_ARGUMENT` if `node_namespace` is not valid, by validate_namespace() definition,
    /// or return `RET_INVALID_ARGUMENT` if `topic_names_and_types` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_NODE_NAME_NON_EXISTENT` if the node name wasn't found,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_publisher_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all service names and types for which a given remote node has servers.
    ///
    /// This function returns an array of service names and types for which a given remote
    /// node has servers, as discovered so far by the given local node.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying served service names and types:
    /// - Access to the array of names and types is not synchronized. It is not safe to read or write
    ///   `service_names_and_types` while get_service_names_and_types() uses it.
    /// - Access to node name and namespace is read-only but it is not synchronized.
    ///   Concurrent `node_name` and `node_namespace` reads are safe, but concurrent reads and
    ///   writes are not.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `node_name` is not valid, by validate_node_name() definition,
    /// or return `RET_INVALID_ARGUMENT` if `node_namespace` is not valid, by validate_namespace() definition,
    /// or return `RET_INVALID_ARGUMENT` if `service_names_and_types` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `service_names_and_types` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_NODE_NAME_NON_EXISTENT` if the node name wasn't found,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_service_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        service_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all service names and types for which a given remote node has clients.
    ///
    /// This function returns an array of service names and types for which a given remote
    /// node has clients, as discovered so far by the given local node.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying served service names and types:
    /// - Access to the array of names and types is not synchronized.
    ///   It is not safe to read or write `service_names_and_types`
    ///   while get_client_names_and_types_by_node() uses it.
    /// - Access to C-style string arguments is read-only but it is not synchronized.
    ///   Concurrent `node_name` and `node_namespace` reads are safe, but concurrent reads and
    ///   writes are not.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `node_name` is not valid, by validate_node_name() definition,
    /// or return `RET_INVALID_ARGUMENT` if `node_namespace` is not valid, by validate_namespace() definition,
    /// or return `RET_INVALID_ARGUMENT` if `service_names_and_types` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `service_names_and_types` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_NODE_NAME_NON_EXISTENT` if the node name wasn't found,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_client_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        service_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all service names and types in the ROS graph.
    ///
    /// This function returns an array of all service names and types in the ROS graph
    /// i.e. for which a server and/or client exists, as discovered so far by the given
    /// local node.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying services names and types:
    /// - Access to the array of names and types is not synchronized.
    ///   It is not safe to read or write `service_names_and_types`
    ///   while get_service_names_and_types() uses it.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    ///
    /// Given `services_names_and_types` must be a zero-initialized array of names and types,
    /// as returned by get_zero_initialized_names_and_types().
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `service_names_and_types` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `service_names_and_types` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_service_names_and_types(
        node: &Node,
        service_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all topic names and types in the ROS graph.
    ///
    /// This function returns an array of all topic names and types in the ROS graph
    /// i.e. for which a publisher and/or a subscription exists, as discovered so far
    /// by the given local node.
    ///
    /// Unless `no_demangle` is true, some demangling and filtering may take place when
    /// listing topics as implemented by the middleware.
    /// Whether demangling applies or not, and how it applies, depends on the underlying
    /// implementation.
    /// See http://design.ros2.org/articles/topic_and_service_names.html for an example
    /// on how it is used in DDS and RTPS based implementations.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying topic names and types:
    /// - Access to the array of names and types is not synchronized.
    /// It is not safe to read or write `topic_names_and_types`
    /// while get_topic_names_and_types() uses it.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    /// Check your allocator documentation for further reference.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    /// Given `topic_names_and_types` must be a zero-initialized array of names and types,
    /// as returned by get_zero_initialized_names_and_types().
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `topic_names_and_types` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `topic_names_and_types` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_topic_names_and_types(
        node: &Node,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Retrieve endpoint information for each known publisher of a given topic.
    ///
    /// This function returns an array of endpoint information for each publisher
    /// of a given topic, as discovered so far by the given node.
    /// Endpoint information includes the publisher's node name and namespace,
    /// the associated topic type, the publisher's gid, and the publisher QoS profile.
    /// Names of non-existent topics are allowed, in which case an empty array will be returned.
    ///
    /// QoS that are correctly read
    /// The QoS profiles returned might have some invalid fields.
    /// The rmw implementation must set the invalid fields to `QOS_POLICY_*_UNKNOWN`.
    /// For DDS based implementations, the only QoS policies that are guaranteed to be shared
    /// during discovery are the ones that participate in endpoint matching.
    /// From the current QoS settings available, the only ones not shared by DDS based
    /// implementations are `history` and `history_depth`.
    ///
    /// Runtime behavior:
    ///
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    ///
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying topic names and types:
    /// - Access to the array of topic endpoint information is not synchronized.
    ///   It is not safe to read or write `publishers_info`
    ///   while get_publishers_info_by_topic() uses it.
    /// - Access to C-style string arguments is read-only but it is not synchronized.
    ///   Concurrent `topic_name` reads are safe, but concurrent reads and writes are not.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    ///
    /// Given `publishers_info` must be a zero-initialized array of endpoints' information,
    /// as returned by get_zero_initialized_topic_endpoint_info_array().
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `node` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `topic_name` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `publishers_info` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `publishers_info` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_publishers_info_by_topic(
        node: &Node,
        topic_name: &str,
        no_mangle: bool,
        publishers_info: &mut TopicEndpointInfoArray,
    ) -> RetType;

    /// Retrieve endpoint information for each known subscription of a given topic.
    ///
    /// This function returns an array of endpoint information for each subscription
    /// of a given topic, as discovered so far by the given node.
    /// Endpoint information includes the subscription's node name and namespace,
    /// the associated topic type, the subscription's gid, and the subscription QoS profile.
    /// Names of non-existent topics are allowed, in which case an empty array will be returned.
    ///
    /// QoS that are correctly read
    /// Not all QoS may be read correctly, see also get_publishers_info_by_topic() for more details.
    ///
    /// Runtime behavior:
    /// To query the ROS graph is a synchronous operation.
    /// It is also non-blocking, but it is not guaranteed to be lock-free.
    /// Generally speaking, implementations may synchronize access to internal resources using
    /// locks but are not allowed to wait for events with no guaranteed time bound (barring
    /// the effects of starvation due to OS scheduling).
    ///
    /// Thread-safety:
    /// Nodes are thread-safe objects, and so are all operations on them except for finalization.
    /// Therefore, it is safe to query the ROS graph using the same node concurrently.
    /// However, when querying subscriptions' information:
    /// - Access to the array of topic endpoint information is not synchronized.
    ///   It is not safe to read or write `subscriptions_info`
    ///   while get_subscriptions_info_by_topic() uses it.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    ///   Check your allocator documentation for further reference.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    ///
    /// Given `subscriptions_info` must be a zero-initialized array of endpoints' information,
    /// as returned by get_zero_initialized_topic_endpoint_info_array().
    ///
    /// Return `RET_OK` if the query was successful,
    /// or return `RET_INVALID_ARGUMENT` if `topic_name` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `subscriptions_info` is NULL,
    /// or return `RET_INVALID_ARGUMENT` if `subscriptions_info` is not a zero-initialized array,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the `node` implementation identifier
    /// does not match this implementation,
    /// or return `RET_ERROR` if an unspecified error occurs.
    fn get_subscriptions_info_by_topic(
        node: &Node,
        topic_name: &str,
        no_mangle: bool,
        subscriptions_info: &mut TopicEndpointInfoArray,
    ) -> RetType;

    /// Finalize a given node handle, reclaim the resources, and deallocate the node handle.
    ///
    /// This function will return early if a logical error, such as `RET_INVALID_ARGUMENT`
    /// or `RET_INCORRECT_RMW_IMPLEMENTATION`, ensues, leaving the given node handle unchanged.
    /// Otherwise, it will proceed despite errors, freeing as many resources as it can, including
    /// the node handle. Usage of a deallocated node handle is undefined behavior.
    ///
    /// All publishers, subscribers, services, and clients created from this node must
    /// have been destroyed prior to this call. Some rmw implementations may verify this,
    /// returning `RET_ERROR` and setting a human readable error message if any entity
    /// created from this node has not yet been destroyed. However, this is not guaranteed
    /// and so callers should ensure that this is the case before calling this function.
    ///
    /// Return `RET_OK` if successful,
    /// or return `RET_INVALID_ARGUMENT` if node is invalid,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if the implementation identifier does not match,
    /// or return `RET_ERROR` if an unexpected error occurs.
    fn destroy_node(node: &mut Node) -> RetType;

    /// Return a guard condition which is triggered when the ROS graph changes.
    ///
    /// The guard condition will be triggered anytime a change to the ROS graph occurs.
    /// A ROS graph change occurs whenever:
    /// - A node joins or leaves the ROS graph.
    ///   This change will be reflected in get_node_names() and
    ///   get_node_names_with_enclaves() outcome.
    /// - A topic subscription joins or leaves the ROS graph.
    ///   This change will be reflected in get_topic_names_and_types(),
    ///   get_subscriber_names_and_types_by_node(), and get_subscriptions_info_by_topic() outcome.
    /// - A topic publisher joins or leaves the ROS graph.
    ///   This change will be reflected in get_topic_names_and_types(),
    ///   get_publisher_names_and_types_by_node(), and get_publishers_info_by_topic() outcome.
    /// - A topic subscription matches a topic publisher with compatible QoS policies.
    ///   This change will be reflected in subscription_count_matched_publishers() outcome.
    /// - A topic publisher matches a topic subscription with compatible QoS policies.
    ///   This change will be reflected in publisher_count_matched_subscriptions() outcome.
    /// - A service server joins or leaves the ROS graph.
    ///   This change will be reflected in get_service_names_and_types() and
    ///   get_service_names_and_types_by_node() outcome.
    /// - A service client joins or leaves the ROS graph.
    ///   This change will be reflected in get_service_names_and_types() and
    ///   get_client_names_and_types_by_node() outcome.
    /// - A service client matches a service server with compatible QoS policies.
    ///   This change will be reflected in service_server_is_available() outcome.
    ///
    /// The state of the ROS graph, and any changes that may take place,
    /// are reported as seen by the associated `node`.
    ///
    /// The guard condition is owned and internally held by the `node`.
    /// It will be invalidated if `node` is finalized using destroy_node().
    /// It is undefined behavior to use an invalidated guard condition.
    ///
    /// Given `node` must be a valid node handle, as returned by create_node().
    ///
    /// Return Guard condition if successful, or `NULL` if `node` is `NULL`,
    /// or an unspecified error occurs.
    // TODO(Shaohua): Returns Option<Rc<Box<GuardCondition>>>
    fn get_graph_guard_condition(node: &Node) -> Option<Box<GuardCondition>>;

    /// Create a publisher and return a handle to that publisher.
    ///
    /// This function can fail, and therefore return `NULL`, if:
    /// - node is not a valid non-null handle for this rmw implementation,
    ///   as returned by `create_node()`
    /// - type_support is a not valid non-null message type support, as returned by
    ///   `ROSIDL_GET_MSG_TYPE_SUPPORT()`
    /// - topic_name is not a valid non-null topic name, according to `validate_full_topic_name()`
    /// - qos_profile is not a fully specified non-null profile i.e. no UNKNOWN policies
    /// - publisher_options is not a valid non-null option set, as returned by
    ///   `get_default_publisher_options()`
    /// - memory allocation fails during publisher creation
    /// - an unspecified error occurs
    ///
    /// Return rmw publisher handle, or `NULL` if there was an error.
    fn create_publisher(
        node: &Node,
        // TODO(Shaohua):
        //const rosidl_message_type_support_t * type_support,
        topic_name: &str,
        qos_profile: &QoSProfile,
        publisher_options: &PublisherOptions,
    ) -> Option<Publisher>;

    /// Finalize a given publisher handle, reclaim the resources, and deallocate the publisher handle.
    ///
    /// This function will return early if a logical error, such as `RET_INVALID_ARGUMENT`
    /// or `RET_INCORRECT_RMW_IMPLEMENTATION`, ensues, leaving the given publisher handle unchanged.
    /// Otherwise, it will proceed despite errors, freeing as many resources as it can, including
    /// the publisher handle. Usage of a deallocated publisher handle is undefined behavior.
    ///
    /// Given node must be the one the publisher was registered with.
    ///
    /// Return `RET_OK` if successful,
    /// or return `RET_INVALID_ARGUMENT` if node or publisher is `NULL`,
    /// or return `RET_INCORRECT_RMW_IMPLEMENTATION` if node or publisher implementation identifier
    /// does not match,
    /// or return `RET_ERROR` if an unexpected error occurs.
    fn destroy_publisher(node: &mut Node, publisher: &mut Publisher) -> RetType;

    /// Create a subscription and return a handle to that subscription.
    //TODO(Shaohua): Replace Option<T> with Result<T>.
    fn create_subscription(
        node: &Node,
        //const rosidl_message_type_support_t * type_support,
        topic_name: &str,
        qos_policies: &QoSProfile,
        subscription_options: &SubscriptionOptions,
    ) -> Option<Subscription>;

    /// Finalize a given subscription handle, reclaim the resources, and deallocate the subscription handle.
    fn destroy_subscription(node: &mut Node, subscription: &mut Subscription) -> RetType;
}
