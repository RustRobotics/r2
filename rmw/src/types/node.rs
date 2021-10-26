// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::Context;
use crate::names_and_types::NamesAndTypes;
use crate::ret_types::RetType;

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
    fn get_subscriber_names_and_types(
        &self,
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
    fn get_publisher_names_and_types(
        &self,
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
    fn get_service_names_and_types(
        &self,
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
    fn get_client_names_and_types(
        &self,
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
    fn get_all_service_names_and_types(
        &self,
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
    fn get_all_topic_names_and_types(
        &self,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;
}
