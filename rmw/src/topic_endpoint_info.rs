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
    pub fn set_topic_type(&mut self, topic_type: &str) -> ret_types::RetType {
        if topic_type.is_empty() {
            log::error!("topic_type is empty");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        self.topic_type.clear();
        self.topic_type.push_str(topic_type);
        ret_types::RET_OK
    }
    /// Set the node name in the given topic endpoint info data structure.
    ///
    /// This functions allocates memory and copies the value of the `node_name`
    /// argument to set the data structure `node_name` member.
    pub fn set_node_name(&mut self, node_name: &str) -> ret_types::RetType {
        if node_name.is_empty() {
            log::error!("node_name is empty");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        self.node_name.clear();
        self.node_name.push_str(node_name);
        ret_types::RET_OK
    }
    /// Set the endpoint type in the given topic endpoint info data structure.
    ///
    /// This functions assigns the value of the `type` argument to the data structure
    /// `endpoint_type` member.
    pub fn set_endpoint_type(&mut self, endpoint_type: EndpointType) -> ret_types::RetType {
        self.endpoint_type = endpoint_type;
        ret_types::RET_OK
    }

    /// Set the endpoint gid in the given topic endpoint info data structure.
    ///
    /// This functions copies the value of the `gid` argument to the data structure
    /// `endpoint_gid` member.
    pub fn set_gid(&mut self, gid: &[u8]) -> ret_types::RetType {
        if gid.len() > GID_STORAGE_SIZE {
            log::error!("size is more than GID_STORAGE_SIZE");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        self.endpoint_gid[0..gid.len()].copy_from_slice(gid);
        ret_types::RET_OK
    }

    /// Set the endpoint QoS profile in the given topic endpoint info data structure.
    ///
    /// This functions assigns the value of the `qos_profile` argument to the data structure
    /// `qos_profile` member.
    pub fn set_qos_profile(&mut self, qos_profile: &QoSProfile) -> ret_types::RetType {
        self.qos_profile = qos_profile.clone();
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
