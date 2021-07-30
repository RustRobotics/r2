// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use rmw::qos_profiles::QoSProfile;

/// Internal rcl publisher implementation struct.
pub trait PublisherImpl: Debug {}

/// Structure which encapsulates a R2 Publisher.
#[derive(Debug)]
pub struct Publisher {
    /// Pointer to the publisher implementation
    pub imp: Box<dyn PublisherImpl>,
}

/// Options available for a rcl publisher.
#[derive(Debug)]
pub struct PublisherOptions {
    /// Middleware quality of service settings for the publisher.
    pub qos: QoSProfile,

    /// rmw specific publisher options, e.g. the rmw implementation specific payload.
    pub rmw_publisher_options: rmw::types::PublisherOptions,
}
