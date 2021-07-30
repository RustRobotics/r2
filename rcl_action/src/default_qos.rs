// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rmw::qos_profiles::QoSProfile;
use rmw::time::duration_unspecified;
use rmw::types::{
    QoSDurabilityPolicy, QoSHistoryPolicy, QoSLivelinessPolicy, QoSReliabilityPolicy,
};

pub fn qos_profile_status_default() -> QoSProfile {
    QoSProfile {
        history: QoSHistoryPolicy::KeepLast,
        depth: 1,
        reliability: QoSReliabilityPolicy::Reliable,
        durability: QoSDurabilityPolicy::TransientLocal,
        deadline: duration_unspecified(),
        lifespan: duration_unspecified(),
        liveliness: QoSLivelinessPolicy::SystemDefault,
        liveliness_lease_duration: duration_unspecified(),
        avoid_r2_namespace_conventions: false,
    }
}
