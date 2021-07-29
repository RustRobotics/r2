// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use super::init_options::InitOptions;

/// Initialization context structure which is used to store init specific information.
#[derive(Debug)]
pub struct Context {
    /// Locally (process local) unique ID that represents this init/shutdown cycle.
    pub instance_id: u64,

    /// Implementation identifier, used to ensure two different implementations are not being mixed.
    pub implementation_identifier: String,

    /// Options used to initialize the context.
    pub options: InitOptions,

    /// Domain id that is being used.
    pub actual_domain_id: usize,

    /// Implementation defined context information.
    /// May be NULL if there is no implementation defined context information.
    pub imp: Box<dyn ContextImpl>,
}

pub trait ContextImpl: Debug {}
