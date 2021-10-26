// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::network_flow_endpoint::NetworkFlowEndpoint;
use crate::ret_types;

/// Structure to hold an arrary of NetworkFlowEndpoint.
#[derive(Debug, Default)]
pub struct NetworkFlowEndpointArray(Vec<NetworkFlowEndpoint>);

impl NetworkFlowEndpointArray {
    /// return a instance with zero-initialized members.
    pub fn zero_initialized() -> Self {
        Self::default()
    }

    /// Check if instance is zero-initialized.
    pub fn check_zero(&self) -> ret_types::RetType {
        if !self.0.is_empty() {
            log::error!("NetworkFlowEndpointArray is not zeroed");
            return ret_types::RET_ERROR;
        }
        ret_types::RET_OK
    }
}
