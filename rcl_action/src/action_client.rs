// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use rmw::qos_profiles::QoSProfile;

/// Internal action client implementation struct.
pub trait ActionClientImpl: Debug {}

/// Structure which encapsulates a ROS action client.
#[derive(Debug)]
pub struct ActionClient {
    /// Pointer to the action client implementation
    pub imp: Box<dyn ActionClientImpl>,
}

/// Options available for a rcl_action_client_t.
#[derive(Debug)]
pub struct ActionClientOptions {
    /// Middleware quality of service settings for the action client.
    /// Goal service quality of service
    pub goal_service_qos: QoSProfile,

    /// Result service quality of service
    pub result_service_qos: QoSProfile,

    /// Cancel service quality of service
    pub cancel_service_qos: QoSProfile,

    /// Feedback topic quality of service
    pub feedback_topic_qos: QoSProfile,

    /// Status topic quality of service
    pub status_topic_qos: QoSProfile,
}
