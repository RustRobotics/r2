// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSPolicyKind {
    Invalid = 0,
    Deadline,
    Liveliness,
    Reliability,
    History,
    Lifespan,
    Depth,
    LivelinessLeaseDuration,
    AvoidR2NamespaceConventions,
}
