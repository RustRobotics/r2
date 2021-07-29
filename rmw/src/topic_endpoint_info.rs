// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::qos_profiles::QoSProfile;
use super::types::{EndpointType, GID_STORAGE_SIZE};

/// A data structure that encapsulates the node name, node namespace,
/// topic_type, gid, and qos_profile of publishers and subscriptions
/// for a topic.
#[derive(Debug)]
pub struct TopicEndpointInfo {
    /// Name of the node
    pub node_name: String,

    /// Namespace of the node
    pub node_namespace: String,

    /// The associated topic type
    pub topic_type: String,

    /// The endpoint type
    pub endpoint_type: EndpointType,

    /// The GID of the endpoint
    pub endpoint_gid: [u8; GID_STORAGE_SIZE],

    /// QoS profile of the endpoint
    pub qos_profile: QoSProfile,
}
