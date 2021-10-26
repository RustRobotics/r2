// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

use crate::domain_id::{DomainId, DEFAULT_DOMAIN_ID};
use crate::localhost::LocalhostOnly;
use crate::ret_types::RetType;
use crate::security_options::SecurityOptions;

/// Options structure used during rmw_init().
#[derive(Debug)]
pub struct InitOptions {
    /// Locally (process local) unique ID that represents this init/shutdown cycle.
    ///
    /// This should be set by the caller of `rmw_init()` to a number that is
    /// unique within this process.
    /// It is designed to be used with `rcl_init()` and `rcl_get_instance_id()`.
    pub instance_id: u64,

    /// Implementation identifier, used to ensure two different implementations are not being mixed.
    pub implementation_identifier: String,

    /// R2 domain id
    pub domain_id: DomainId,

    /// Security options
    pub security_options: SecurityOptions,

    /// Enable localhost only
    pub localhost_only: LocalhostOnly,

    /// Enclave, name used to find security artifacts in a sr2 keystore.
    pub enclave: String,

    /// Implementation defined init options.
    ///
    /// May be NULL if there are no implementation defined options.
    pub imp: Option<Box<dyn InitOptionsImpl>>,
}

/// Implementation defined options structure used during rmw_init().
///
/// This should be defined by the rmw implementation.
pub trait InitOptionsImpl: fmt::Debug {}

impl InitOptions {
    /// Return a zero initialized init options structure.
    pub fn zero_initialized() -> Self {
        Self::default()
    }
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            domain_id: DEFAULT_DOMAIN_ID,
            localhost_only: LocalhostOnly::UseDefault,
            implementation_identifier: String::new(),
            imp: None,
            instance_id: 0,
            enclave: String::new(),
            security_options: SecurityOptions::default(),
        }
    }
}

pub trait InitOptionsTrait {
    /// Initialize given init options with the default values and implementation specific values.
    ///
    /// The `impl` pointer should not be changed manually.
    /// The given init options must be zero initialized.
    /// If initialization fails, init options will remain zero initialized.
    /// Giving an already initialized init options will result in a failure
    /// with return code `RMW_RET_INVALID_ARGUMENT`.
    fn init(&mut self) -> RetType;

    /// Copy the given source init options to the destination init options.
    fn copy(&self, dest: &mut Self) -> RetType;
}
