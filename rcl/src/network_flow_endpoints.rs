// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rmw::{network_flow_endpoint, network_flow_endpoint_array};

pub type NetworkFlowEndpoint = network_flow_endpoint::NetworkFlowEndpoint;
pub type NetworkFlowEndpointArray = network_flow_endpoint_array::NetworkFlowEndpointArray;
pub type TransportProtocol = network_flow_endpoint::TransportProtocol;
pub type InternetProtocol = network_flow_endpoint::InternetProtocol;
