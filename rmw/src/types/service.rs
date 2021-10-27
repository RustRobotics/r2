// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::ret_types::RetType;
use crate::types::{RequestId, ServiceInfo};

/// A handle to an rmw service
#[derive(Debug)]
pub struct Service {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this service
    //void * data;
    pub data: *const u8,

    /// The name of this service as exposed to the r2 graph
    pub service_name: String,
}

pub trait ServiceTrait {
    /// Take an incoming ROS service request.
    fn take_request(
        service: &Service,
        request_header: &mut ServiceInfo,
        ros_request: usize,
        taken: &mut bool,
    ) -> RetType;

    /// Send a ROS service response.
    fn send_response(
        service: &Service,
        request_header: &mut RequestId,
        ros_response: usize,
    ) -> RetType;
}
