// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use super::context::Context;

pub trait NodeImpl: Debug {}

/// Structure which encapsulates a R2 Node.
#[derive(Debug)]
pub struct Node {
    /// Context associated with this node.
    pub context: Box<Context>,

    /// Private implementation pointer.
    pub imp: Box<dyn NodeImpl>,
}
