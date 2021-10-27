// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::SubscriptionOptions;
use crate::message_sequence::{MessageInfoSequence, MessageSequence};
use crate::network_flow_endpoint_array::NetworkFlowEndpointArray;
use crate::qos_profiles::QoSProfile;
use crate::ret_types::RetType;
use crate::serialized_message::SerializedMessage;
use crate::types::MessageInfo;

#[derive(Debug)]
pub struct Subscription {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this subscription
    //void * data;
    pub data: usize,

    /// Name of the r2 topic this subscription listens to
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

pub trait SubscriptionTrait {
    /// Get network flow endpoints of a subscription.
    ///
    /// Query the underlying middleware for a given subscription's network flow endpoints.
    /// return `RET_OK` if successful, or return `RET_INVALID_ARGUMENT` if any argument is null,
    /// or return `RET_UNSUPPORTED` if not supported, or return `RET_ERROR` if an unexpected error occurs.
    fn get_network_flow_endpoints(
        subscription: &Subscription,
        array: &mut NetworkFlowEndpointArray,
    ) -> RetType;

    /// Retrieve the number of matched publishers to a subscription.
    fn count_matched_publishers(
        subscription: &Subscription,
        publisher_count: &mut usize,
    ) -> RetType;

    /// Retrieve the actual qos settings of the subscription.
    fn get_actual_qos(subscription: &Subscription, qos: &mut QoSProfile) -> RetType;

    /// Take an incoming ROS message.
    fn take(subscription: &Subscription, ros_message: usize, taken: &mut bool) -> RetType;

    /// Take an incoming ROS message with its metadata.
    fn take_with_info(
        subscription: &Subscription,
        ros_message: usize,
        taken: &mut bool,
        message_info: &mut MessageInfo,
    ) -> RetType;

    /// Take multiple incoming ROS messages with their metadata.
    fn take_sequence(
        subscription: &Subscription,
        count: usize,
        message_sequence: &mut MessageSequence,
        message_info_sequence: &mut MessageInfoSequence,
        taken: &mut usize,
    ) -> RetType;

    /// Take an incoming ROS message as a byte stream.
    fn take_serialized_message(
        subscription: &Subscription,
        serialized_message: &mut SerializedMessage,
        taken: &mut bool,
    ) -> RetType;

    /// Take an incoming ROS message as a byte stream with its metadata.
    fn take_serialized_message_with_info(
        subscription: &Subscription,
        serialized_message: &mut SerializedMessage,
        taken: &mut bool,
        message_info: &mut MessageInfo,
    ) -> RetType;

    /// Take an incoming ROS message, loaned by the middleware.
    fn take_loaned_message(
        subscription: &Subscription,
        loaned_message: usize,
        taken: &mut bool,
    ) -> RetType;

    /// Take a loaned message and with its additional message information.
    fn take_loaned_message_with_info(
        subscription: &Subscription,
        loaned_message: usize,
        taken: &mut bool,
        message_info: &mut MessageInfo,
    ) -> RetType;

    /// Return a loaned ROS message previously taken from a subscription.
    fn return_loaned_message_from_subscription(
        subscription: &Subscription,
        loaned_message: usize,
    ) -> RetType;
}
