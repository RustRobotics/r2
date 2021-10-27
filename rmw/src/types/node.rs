// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::Context;
use crate::names_and_types::NamesAndTypes;
use crate::qos_profiles::QoSProfile;
use crate::ret_types::RetType;
use crate::topic_endpoint_info_array::TopicEndpointInfoArray;
use crate::types::{
    Client, GuardCondition, Publisher, PublisherOptions, Service, Subscription, SubscriptionOptions,
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
    fn get_subscriber_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all topic names and types for which a given remote node has publishers.
    fn get_publisher_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all service names and types for which a given remote node has servers.
    fn get_service_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        service_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all service names and types for which a given remote node has clients.
    fn get_client_names_and_types_by_node(
        node: &Node,
        node_name: &str,
        node_namespace: &str,
        service_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all service names and types in the ROS graph.
    fn get_service_names_and_types(
        node: &Node,
        service_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Return all topic names and types in the ROS graph.
    fn get_topic_names_and_types(
        node: &Node,
        no_demangle: bool,
        topic_names_and_types: &mut NamesAndTypes,
    ) -> RetType;

    /// Retrieve endpoint information for each known publisher of a given topic.
    fn get_publishers_info_by_topic(
        node: &Node,
        topic_name: &str,
        no_mangle: bool,
        publishers_info: &mut TopicEndpointInfoArray,
    ) -> RetType;

    /// Retrieve endpoint information for each known subscription of a given topic.
    fn get_subscriptions_info_by_topic(
        node: &Node,
        topic_name: &str,
        no_mangle: bool,
        subscriptions_info: &mut TopicEndpointInfoArray,
    ) -> RetType;

    /// Finalize a given node handle, reclaim the resources, and deallocate the node handle.
    fn destroy_node(node: &mut Node) -> RetType;

    /// Return a guard condition which is triggered when the ROS graph changes.
    // TODO(Shaohua): Returns Option<Rc<Box<GuardCondition>>>
    fn get_graph_guard_condition(node: &Node) -> Option<Box<GuardCondition>>;

    /// Create a publisher and return a handle to that publisher.
    fn create_publisher(
        node: &Node,
        // TODO(Shaohua):
        //const rosidl_message_type_support_t * type_support,
        topic_name: &str,
        qos_profile: &QoSProfile,
        publisher_options: &PublisherOptions,
    ) -> Option<Publisher>;

    /// Finalize a given publisher handle, reclaim the resources, and deallocate the publisher handle.
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

    /// Create a service client that can send requests to and receive replies from a service server.
    fn create_client(
        node: &mut Node,
        //const rosidl_service_type_support_t * type_support,
        service_name: &str,
        qos_policies: &QoSProfile,
    ) -> RetType;

    /// Destroy and unregister a service client from its node.
    fn destroy_client(node: &mut Node, client: &mut Client) -> RetType;

    /// Create a service server that can receive requests from and send replies to a service client.
    fn create_service(
        node: &mut Node,
        //const rosidl_service_type_support_t * type_support,
        service_name: &str,
        qos_profile: &QoSProfile,
    ) -> Option<Service>;

    /// Destroy and unregister a service server from its node.
    fn destroy_service(node: &mut Node, service: &mut Service) -> RetType;

    /// Return the name and namespace of all nodes in the R2 graph.
    fn get_node_names(
        node: &Node,
        node_names: &mut Vec<String>,
        node_namespaces: &mut Vec<String>,
    ) -> RetType;

    /// Return the name, namespae, and enclave name of all nodes in the R2 graph.
    fn get_node_names_with_enclaves(
        node: &Node,
        node_names: &mut Vec<String>,
        node_namespaces: &mut Vec<String>,
        enclaves: &mut Vec<String>,
    ) -> RetType;

    /// Count the number of known publishers matching a topic name.
    fn count_publishers(node: &Node, topic_name: &str, count: &mut usize) -> RetType;

    /// Count the number of known subscribers matching a topic name.
    fn count_subscribers(node: &Node, topic_name: &str, count: &mut usize) -> RetType;

    /// Check if a service server is available for the given service client.
    fn service_server_is_available(
        node: &Node,
        client: &Client,
        is_available: &mut bool,
    ) -> RetType;
}
