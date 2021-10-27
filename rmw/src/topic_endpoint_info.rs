// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::qos_profiles::QoSProfile;
use crate::ret_types;
use crate::types::{EndpointType, GID_STORAGE_SIZE};

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

impl TopicEndpointInfo {
    /// Return zero initialized topic endpoint info data structure.
    ///
    /// Endpoint type will be invalid.
    /// Endpoint QoS profile will be the system default.
    pub fn zero_initialized() -> Self {
        Self::default()
    }

    /// Set the topic type in the given topic endpoint info data structure.
    ///
    /// This functions allocates memory and copies the value of the `topic_type`
    /// argument to set the data structure `topic_type` member.
    ///
    /// Thread-safety:
    ///
    /// Setting a member is a reentrant procedure, but:
    /// - Access to the topic endpoint info data structure is not synchronized.
    /// It is not safe to read or write the `topic_type` member of the given `topic_endpoint`
    /// while setting it.
    /// - Access to C-style string arguments is read-only but it is not synchronized.
    /// Concurrent `topic_type` reads are safe, but concurrent reads and writes are not.
    /// - The default allocators are thread-safe objects, but any custom `allocator` may not be.
    /// Check your allocator documentation for further reference.
    ///
    /// Returns `RET_OK` if successful,
    /// or returns `RET_INVALID_ARGUMENT` if `topic_endpoint_info` is NULL,
    /// or returns `RET_INVALID_ARGUMENT` if `topic_type` is NULL,
    /// or returns `RET_ERROR` when an unspecified error occurs.
    ///
    /// This function sets the RMW error state on failure.
    pub fn set_topic_type(&mut self, topic_type: &str) -> ret_types::RetType {
        if topic_type.is_empty() {
            log::error!("str is null");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        self.topic_type.clear();
        self.topic_type.push_str(topic_type);
        ret_types::RET_OK
    }
}

impl Default for TopicEndpointInfo {
    fn default() -> Self {
        Self {
            node_name: String::new(),
            node_namespace: String::new(),
            topic_type: String::new(),
            endpoint_type: EndpointType::Invalid,
            endpoint_gid: [0; GID_STORAGE_SIZE],
            qos_profile: QoSProfile::system_default(),
        }
    }
}

pub trait TopicEndpointInfoTrait {}
