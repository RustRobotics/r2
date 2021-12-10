// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::init::ContextTrait;
use crate::names_and_types::NamesAndTypes;
use crate::qos_profiles::QoSProfile;
use crate::ret_types::RetType;
use crate::topic_endpoint_info_array::TopicEndpointInfoArray;
use crate::types::{
    Client, GuardCondition, Publisher, PublisherOptions, Service, Subscription, SubscriptionOptions,
};

/// Structure which encapsulates an rmw node
pub trait NodeBaseTrait<'a> {
    /// Name of the rmw implementation
    fn implementation_identifier(&self) -> &'a str;

    /// Type erased pointer to this node's data.
    fn data(&self) -> &'a [u8];

    /// A concise name of this rmw node for identification.
    fn name(&self) -> &'a str;

    /// The namespace of this rmw node.
    fn namespace_(&self) -> &'a str;

    /// Context information about node's init specific information.
    fn context(&self) -> Option<&dyn ContextTrait>;
}

pub trait NodeTrait<'a>: NodeBaseTrait<'a> {
    /// Return all topic names and types for which a given remote node has subscriptions.
    fn get_subscriber_names_and_types_by_node(
        &self,
        node_name: &str,
        node_namespace: &str,
        no_demangle: bool,
    ) -> Result<&NamesAndTypes, RetType>;

    /// Return all topic names and types for which a given remote node has publishers.
    fn get_publisher_names_and_types_by_node(
        &self,
        node_name: &str,
        node_namespace: &str,
        no_demangle: bool,
    ) -> Result<&NamesAndTypes, RetType>;

    /// Return all service names and types for which a given remote node has servers.
    fn get_service_names_and_types_by_node(
        &self,
        node_name: &str,
        node_namespace: &str,
    ) -> Result<&NamesAndTypes, RetType>;

    /// Return all service names and types for which a given remote node has clients.
    fn get_client_names_and_types_by_node(
        &self,
        node_name: &str,
        node_namespace: &str,
    ) -> Result<&NamesAndTypes, RetType>;

    /// Return all service names and types in the ROS graph.
    fn get_service_names_and_types(&self) -> Result<&NamesAndTypes, RetType>;

    /// Return all topic names and types in the ROS graph.
    fn get_topic_names_and_types(&self, no_demangle: bool) -> Result<&NamesAndTypes, RetType>;

    /// Retrieve endpoint information for each known publisher of a given topic.
    fn get_publishers_info_by_topic(
        &self,
        topic_name: &str,
        no_mangle: bool,
    ) -> Result<&TopicEndpointInfoArray, RetType>;

    /// Retrieve endpoint information for each known subscription of a given topic.
    fn get_subscriptions_info_by_topic(
        &self,
        topic_name: &str,
        no_mangle: bool,
    ) -> Result<&TopicEndpointInfoArray, RetType>;

    /// Finalize a given node handle, reclaim the resources, and deallocate the node handle.
    fn destroy_node(self) -> RetType;

    /// Return a guard condition which is triggered when the ROS graph changes.
    // TODO(Shaohua): Returns Option<Rc<Box<GuardCondition>>>
    fn get_graph_guard_condition(&self) -> Option<Box<GuardCondition>>;

    /// Create a publisher and return a handle to that publisher.
    // TODO(Shaohua):
    //const rosidl_message_type_support_t * type_support,
    fn create_publisher(
        &mut self,
        topic_name: &str,
        qos_profile: &QoSProfile,
        publisher_options: &PublisherOptions,
    ) -> Result<Publisher, RetType>;

    /// Finalize a given publisher handle, reclaim the resources, and deallocate the publisher handle.
    fn destroy_publisher(&mut self, publisher: Publisher) -> RetType;

    /// Create a subscription and return a handle to that subscription.
    // TODO(Shaohua): Replace Option<T> with Result<T>.
    //const rosidl_message_type_support_t * type_support,
    fn create_subscription(
        &mut self,
        topic_name: &str,
        qos_policies: &QoSProfile,
        subscription_options: &SubscriptionOptions,
    ) -> Result<Subscription, RetType>;

    /// Finalize a given subscription handle, reclaim the resources, and deallocate the subscription handle.
    fn destroy_subscription(&mut self, subscription: Subscription) -> RetType;

    /// Create a service client that can send requests to and receive replies from a service server.
    // TODO(Shaohua):
    //const rosidl_service_type_support_t * type_support,
    fn create_client(
        &mut self,
        service_name: &str,
        qos_policies: &QoSProfile,
    ) -> Result<Client, RetType>;

    /// Destroy and unregister a service client from its node.
    fn destroy_client(&mut self, client: Client) -> RetType;

    /// Create a service server that can receive requests from and send replies to a service client.
    // TODO(Shaohua):
    //const rosidl_service_type_support_t * type_support,
    fn create_service(
        &mut self,
        service_name: &str,
        qos_profile: &QoSProfile,
    ) -> Result<Service, RetType>;

    /// Destroy and unregister a service server from its node.
    fn destroy_service(&mut self, service: Service) -> RetType;

    /// Return the name and namespace of all nodes in the R2 graph.
    fn get_node_names(&self) -> Result<(&[String], &[String]), RetType>;

    /// Return the name, namespae, and enclave name of all nodes in the R2 graph.
    fn get_node_names_with_enclaves(&self) -> Result<(&[String], &[String], &[String]), RetType>;

    /// Count the number of known publishers matching a topic name.
    fn count_publishers(&self, topic_name: &str) -> Result<usize, RetType>;

    /// Count the number of known subscribers matching a topic name.
    fn count_subscribers(&self, topic_name: &str) -> Result<usize, RetType>;

    /// Check if a service server is available for the given service client.
    fn service_server_is_available(&self, client: &Client) -> Result<bool, RetType>;
}
