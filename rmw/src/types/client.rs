// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::ServiceInfo;
use crate::ret_types::RetType;

/// A handle to an rmw service client
#[derive(Debug)]
pub struct Client {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this service client
    //void * data;
    pub data: *const u8,

    /// The name of this service as exposed to the r2 graph
    pub service_name: String,
}

pub trait ClientTrait {
    /// Send an R2 service request.
    fn send_request(client: &Client, ros_request: *const u8, sequence_id: &mut i64) -> RetType;

    /// Take an incoming R2 service response.
    fn take_response(
        client: &Client,
        request_header: &mut ServiceInfo,
        ros_response: usize,
        taken: &mut bool,
    ) -> RetType;
}
