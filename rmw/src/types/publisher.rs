// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::PublisherOptions;
use crate::network_flow_endpoint_array::NetworkFlowEndpointArray;
use crate::qos_profiles::QoSProfile;
use crate::ret_types::RetType;
use crate::serialized_message::SerializedMessage;

/// Structure which encapsulates an rmw publisher
#[derive(Debug)]
pub struct Publisher {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this publisher's data
    //void * data;
    pub data: usize,

    /// The name of the R2 topic this publisher publishes to
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

pub trait PublisherTrait {
    /// Get network flow endpoints of a publisher.
    ///
    /// Query the underlying middleware for a given publisher's network flow endpoints.
    /// Return `RET_OK` if successful, or return `RET_INVALID_ARGUMENT` if any argument is null,
    /// return `RET_UNSUPPORTED` if not supported, or return `RET_ERROR` if an unexpected error occurs.
    fn get_network_flow_points(
        publisher: &Publisher,
        array: &mut NetworkFlowEndpointArray,
    ) -> RetType;

    /// Borrow a loaned R2 message.
    fn borrow_loaned_message(
        publisher: &mut Publisher,
        //const rosidl_message_type_support_t * type_support,
        ros_message: usize,
    ) -> RetType;

    /// Return a loaned message previously borrowed from a publisher.
    fn return_loaned_message_from_publisher(
        publisher: &Publisher,
        loaned_message: usize,
    ) -> RetType;

    /// Publish an R2 message.
    fn publish(publisher: &Publisher, ros_message: usize) -> RetType;

    /// Publish a loaned R2 message.
    fn publish_loaned_message(publisher: &Publisher, ros_messge: usize) -> RetType;

    /// Retrieve the number of matched subscriptions to a publisher.
    fn publisher_count_matched_subscriptions(
        publisher: &Publisher,
        subscription_count: &mut usize,
    ) -> RetType;

    /// Retrieve the actual qos settings of the publisher.
    fn publisher_get_actual_qos(publisher: &Publisher, qos: &mut QoSProfile) -> RetType;

    /// Publish a R2 message as a byte stream.
    fn publish_serialized_message(
        publisher: &Publisher,
        serialized_message: &SerializedMessage,
    ) -> RetType;
}
