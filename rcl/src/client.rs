// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use rmw::qos_profiles::QoSProfile;

/// Internal rcl client implementation struct.
pub trait ClientImpl: Debug {}

/// Structure which encapsulates a R2 Client.
#[derive(Debug)]
pub struct Client {
    /// Pointer to the client implementation
    pub imp: Box<dyn ClientImpl>,
}

/// Options available for a Client.
#[derive(Debug)]
pub struct ClientOptions {
    /// Middleware quality of service settings for the client.
    pub qos: QoSProfile,
}
