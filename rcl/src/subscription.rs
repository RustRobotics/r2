// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use rmw::qos_profiles::QoSProfile;

/// Internal rcl implementation struct.
pub trait SubscriptionImpl: Debug {}

/// Structure which encapsulates a ROS Subscription.
#[derive(Debug)]
pub struct Subscription {
    /// Pointer to the subscription implementation
    pub imp: Box<dyn SubscriptionImpl>,
}

/// Options available for a rcl subscription.
#[derive(Debug)]
pub struct SubscriptionOptions {
    /// Middleware quality of service settings for the subscription.
    pub qos: QoSProfile,

    /// rmw specific subscription options, e.g. the rmw implementation specific payload.
    pub rmw_subscription_options: rmw::types::SubscriptionOptions,
}
