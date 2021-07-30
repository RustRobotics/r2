// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;
use std::time::Duration;

use rmw::qos_profiles::QoSProfile;

/// Internal rcl_action implementation struct.
pub trait ActionServerImpl: Debug {}

/// Structure which encapsulates a R2 Action Server.
#[derive(Debug)]
pub struct ActionServer {
    /// Pointer to the action server implementation
    pub imp: Box<dyn ActionServerImpl>,
}

/// Options available for a `ActionServer`.
#[derive(Debug)]
pub struct ActionServerOptions {
    /// Middleware quality of service settings for the action server.
    /// Goal service quality of service
    pub goal_service_qos: QoSProfile,

    /// Cancel service quality of service
    pub cancel_service_qos: QoSProfile,

    /// Result service quality of service
    pub result_service_qos: QoSProfile,

    /// Feedback topic quality of service
    pub feedback_topic_qos: QoSProfile,

    /// Status topic quality of service
    pub status_topic_qos: QoSProfile,

    /// Goal handles that have results longer than this time are deallocated.
    pub result_timeout: Duration,
}
