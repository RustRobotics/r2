// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::{Gid, PublisherOptions};
use crate::network_flow_endpoint_array::NetworkFlowEndpointArray;
use crate::qos_profiles::QoSProfile;
use crate::ret_types::RetType;
use crate::serialized_message::SerializedMessage;

/// Structure which encapsulates an rmw publisher
pub trait PublisherBaseTrait {
    /// Name of the rmw implementation
    fn implementation_identifier(&self) -> &str;

    /// Type erased pointer to this publisher's data
    fn data(&self) -> &[u8];

    /// The name of the R2 topic this publisher publishes to
    fn topic_name(&self) -> &str;

    /// Publisher options.
    ///
    /// The options structure passed to rmw_create_publisher() should be
    /// assigned to this field by the rmw implementation.
    /// The fields should not be modified after creation, but
    /// the contents of the options structure may or may not be const, i.e.
    /// shallow const-ness.
    /// This field is not marked const to avoid any const casting during setup.
    fn options(&self) -> &PublisherOptions;

    /// Indicate whether this publisher supports loaning messages
    fn can_loan_messages(&self) -> bool;
}

pub trait PublisherTrait: PublisherBaseTrait {
    /// Get network flow endpoints of a publisher.
    ///
    /// Query the underlying middleware for a given publisher's network flow endpoints.
    /// Return `RET_OK` if successful, or return `RET_INVALID_ARGUMENT` if any argument is null,
    /// return `RET_UNSUPPORTED` if not supported, or return `RET_ERROR` if an unexpected error occurs.
    fn get_network_flow_points(&self, array: &mut NetworkFlowEndpointArray) -> RetType;

    /// Borrow a loaned R2 message.
    // TODO(Shaohua):
    //const rosidl_message_type_support_t * type_support,
    fn borrow_loaned_message(&self, ros_message: usize) -> RetType;

    /// Return a loaned message previously borrowed from a publisher.
    fn return_loaned_message_from_publisher(&self, loaned_message: usize) -> RetType;

    /// Publish an R2 message.
    fn publish(&mut self, ros_message: usize) -> RetType;

    /// Publish a loaned R2 message.
    fn publish_loaned_message(&mut self, ros_messge: usize) -> RetType;

    /// Retrieve the number of matched subscriptions to a publisher.
    fn count_matched_subscriptions(&self) -> Result<usize, RetType>;

    /// Retrieve the actual qos settings of the publisher.
    fn get_actual_qos(&self) -> Result<&QoSProfile, RetType>;

    /// Publish a R2 message as a byte stream.
    fn publish_serialized_message(&mut self, serialized_message: &SerializedMessage) -> RetType;

    /// Manually assert that this Publisher is alive (for QOS_POLICY_LIVELINESS_MANUAL_BY_TOPIC)
    fn assert_liveliness(&self) -> RetType;

    /// Get the unique identifier (gid) of a publisher.
    fn get_gid_for_publisher(&self) -> Result<&Gid, RetType>;
}
