// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub enum QoSPolicyKind {
    Invalid,
    Deadline,
    Liveliness,
    Reliability,
    History,
    Lifespan,
    Depth,
    LivelinessLeaseDuration,
    AvoidR2NamespaceConventions,
}
