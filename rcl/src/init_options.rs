// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

pub trait InitOptionsImpl: Debug {}

/// Encapsulation of init options and implementation defined init options.
#[derive(Debug)]
pub struct InitOptions {
    /// Implementation specific pointer.
    pub imp: Box<dyn InitOptionsImpl>,
}
