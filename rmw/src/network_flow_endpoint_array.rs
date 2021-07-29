// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::network_flow_endpoint::NetworkFlowEndpoint;

/// Structure to hold an arrary of network_flow_endpoint_t
#[derive(Debug)]
pub struct NetworkFlowEndpointArray(Vec<NetworkFlowEndpoint>);
