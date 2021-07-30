// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use rmw::qos_profiles::QoSProfile;

/// Internal rcl implementation struct.
pub trait ServiceImpl: Debug {}

/// Structure which encapsulates a ROS Service.
#[derive(Debug)]
pub struct Service {
    /// Pointer to the service implementation
    pub imp: Box<dyn ServiceImpl>,
}

/// Options available for a rcl service.
#[derive(Debug)]
pub struct ServiceOptions {
    /// Middleware quality of service settings for the service.
    pub qos: QoSProfile,
}
