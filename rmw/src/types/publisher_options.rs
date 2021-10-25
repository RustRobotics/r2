// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::UniqueNetworkFlowEndpointsRequirement;

/// Options that can be used to configure the creation of a publisher in rmw.
#[derive(Debug)]
pub struct PublisherOptions {
    /// Used to pass rmw implementation specific resources during publisher creation.
    /**
     * This field is type erased (rather than forward declared) because it will
     * usually be a non-owned reference to an language specific object, e.g.
     * C++ it may be a polymorphic class that only the rmw implementation can use.
     *
     * The resource pointed to here needs to outlive this options structure, and
     * any rmw_publisher objects that are created using it, as they copy this
     * structure and may use this payload throughout their lifetime.
     */
    // FIXME(Shaohua):
    //void * rmw_specific_publisher_payload;
    pub rmw_specific_publisher_payload: usize,

    /// Require middleware to generate unique network flow endpoints.
    /**
     * Unique network flow endpoints are required to differentiate the QoS provided by
     * networks for flows between publishers and subscribers in communicating
     * nodes.
     * Default value is RMW_UNIQUE_NETWORK_FLOW_ENDPOINTS_NOT_REQUIRED.
     */
    pub require_unique_network_flow_endpoints: UniqueNetworkFlowEndpointsRequirement,
}

impl Default for PublisherOptions {
    fn default() -> Self {
        Self {
            rmw_specific_publisher_payload: 0,
            require_unique_network_flow_endpoints:
                UniqueNetworkFlowEndpointsRequirement::NotRequired,
        }
    }
}
