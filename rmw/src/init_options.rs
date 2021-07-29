// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use super::localhost::LocalhostOnly;
use super::security_options::SecurityOptions;

/// Options structure used during rmw_init().
#[derive(Debug, Clone)]
pub struct InitOptions {
    /// Locally (process local) unique ID that represents this init/shutdown cycle.
    // This should be set by the caller of `rmw_init()` to a number that is
    // unique within this process.
    // It is designed to be used with `rcl_init()` and `rcl_get_instance_id()`.
    pub instance_id: u64,

    /// Implementation identifier, used to ensure two different implementations are not being mixed.
    pub implementation_identifier: String,

    /// R2 domain id
    pub domain_id: usize,

    /// Security options
    pub security_options: SecurityOptions,

    /// Enable localhost only
    pub localhost_only: LocalhostOnly,

    /// Enclave, name used to find security artifacts in a sr2 keystore.
    pub enclave: String,

    /// Implementation defined init options.
    // May be NULL if there are no implementation defined options.
    pub imp: Box<dyn InitOptionsImpl>,
}

pub trait InitOptionsImpl: Debug + Clone + Sized {}
