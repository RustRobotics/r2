// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rmw::{names_and_types, topic_endpoint_info, topic_endpoint_info_array};

/// A structure that contains topic names and types.
pub type NamesAndTypes = names_and_types::NamesAndTypes;

/// A structure that encapsulates the node name, node namespace,
/// topic type, gid, and qos_profile or publishers and subscriptions
/// for a topic.
pub type TopicEndpointInfo = topic_endpoint_info::TopicEndpointInfo;

/// An array of topic endpoint information.
pub type TopicEndpointInfoArray = topic_endpoint_info_array::TopicEndpointInfoArray;
