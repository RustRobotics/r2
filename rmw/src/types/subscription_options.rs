// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::UniqueNetworkFlowEndpointsRequirement;

/// Options that can be used to configure the creation of a subscription in rmw.
#[derive(Debug)]
pub struct SubscriptionOptions {
    /// Used to pass rmw implementation specific resources during subscription creation.
    ///
    /// All the same details and restrictions of this field in
    /// `PublisherOptions` apply to this struct as well.
    /// rmw_publisher_options_t.rmw_specific_publisher_payload
    // TODO(Shaohua): Replace with fat pointer
    //void * rmw_specific_subscription_payload;
    pub rmw_specific_subscription_payload: usize,

    /// If true then the middleware should not deliver data from local publishers.
    ///
    /// This setting is most often used when data should only be received from
    /// remote nodes, especially to avoid "double delivery" when both intra- and
    /// inter- process communication is taking place.
    ///
    /// The definition of local is somewhat vague at the moment.
    /// Right now it means local to the node, and that definition works best, but
    /// may become more complicated when/if participants map to a context instead.
    pub ignore_local_publications: bool,

    /// Require middleware to generate unique network flow endpoints.
    ///
    /// Unique network flow endpoints are required to differentiate the QoS provided by
    /// networks for flows between publishers and subscribers in communicating
    /// nodes.
    /// Default value is RMW_UNIQUE_NETWORK_FLOW_ENDPOINTS_NOT_REQUIRED.
    pub require_unique_network_flow_endpoints: UniqueNetworkFlowEndpointsRequirement,
}

impl Default for SubscriptionOptions {
    fn default() -> Self {
        Self {
            rmw_specific_subscription_payload: 0,
            ignore_local_publications: false,
            require_unique_network_flow_endpoints:
                UniqueNetworkFlowEndpointsRequirement::NotRequired,
        }
    }
}
