// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::SubscriptionOptions;
use crate::network_flow_endpoint_array::NetworkFlowEndpointArray;
use crate::ret_types::RetType;

#[derive(Debug)]
pub struct Subscription {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this subscription
    //void * data;
    pub data: usize,

    /// Name of the r2 topic this subscription listens to
    pub topic_name: String,

    /// Subscription options.
    ///
    /// The options structure passed to rmw_create_subscription() should be
    /// assigned to this field by the rmw implementation.
    /// The fields should not be modified after creation, but
    /// the contents of the options structure may or may not be const, i.e.
    /// shallow const-ness.
    /// This field is not marked const to avoid any const casting during setup.
    pub options: SubscriptionOptions,

    /// Indicates whether this subscription can loan messages
    pub can_loan_messages: bool,
}

pub trait SubscriptionTrait {
    /// Get network flow endpoints of a subscription.
    ///
    /// Query the underlying middleware for a given subscription's network flow endpoints.
    /// return `RET_OK` if successful, or return `RET_INVALID_ARGUMENT` if any argument is null,
    /// or return `RET_UNSUPPORTED` if not supported, or return `RET_ERROR` if an unexpected error occurs.
    fn get_network_flow_endpoints(&self, array: &mut NetworkFlowEndpointArray) -> RetType;
}
