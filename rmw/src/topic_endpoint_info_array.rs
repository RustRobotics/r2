// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::topic_endpoint_info::TopicEndpointInfo;

/// Array of topic endpoint information
#[derive(Debug)]
pub struct TopicEndpointInfoArray(Vec<TopicEndpointInfo>);
