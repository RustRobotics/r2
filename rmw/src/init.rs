// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

use crate::domain_id::{self, DomainId};
use crate::init_options::{InitOptions, InitOptionsTrait};
use crate::ret_types::{self, RetType};
use crate::types::{GuardCondition, Node, WaitSet};

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
    pub actual_domain_id: DomainId,

    /// Implementation defined context information.
    ///
    /// May be `None` if there is no implementation defined context information.
    pub imp: Option<Box<dyn ContextImpl>>,
}

impl Context {
    /// Return a zero initialized context structure.
    pub fn zero_initialized() -> Self {
        Self::default()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            instance_id: 0,
            implementation_identifier: String::new(),
            options: InitOptions::zero_initialized(),
            actual_domain_id: 0,
            imp: None,
        }
    }
}

/// Implementation defined context structure returned by rmw_init().
///
/// This should be defined by the rmw implementation.
pub trait ContextImpl: fmt::Debug {}

pub trait ContextTrait {
    /// Initialize the middleware with the given options, and yielding an context.
    ///
    /// Context is filled with middleware specific data upon success of this function.
    /// The context is used when initializing some entities like nodes and
    /// guard conditions, and is also required to properly call [`Self::shutdown()`].
    ///
    /// The given options must have been initialized
    /// i.e. [`init()`] called on it and an enclave set.
    ///
    /// The given context must be zero initialized.
    /// If initialization fails, context will remain zero initialized.
    ///
    /// [`context.actual_domain_id`] will be set with the domain id the rmw implementation is using.
    ///
    /// This matches [`options.domain_id`] if it is not [`domain_id::DEFAULT_DOMAIN_ID`].
    /// In other case, the value is rmw implementation dependent.
    ///
    /// If options are zero-initialized, then [`ret_types::RET_INVALID_ARGUMENT`] is returned.
    ///
    /// If options are initialized but no enclave is provided, then [`ret_types::RET_INVALID_ARGUMENT`] is returned.
    ///
    /// If context has been already initialized ([`Self::init()`] was called on it), then
    /// [`ret_types::RET_INVALID_ARGUMENT`] is returned.
    ///
    /// [`init()`]: InitOptionsTrait#tymethod.init
    /// [`context.actual_domain_id`]: Context#structfield.actual_domain_id
    /// [`options.domain_id`]: InitOptions#structfield.domain_id
    fn init(options: &InitOptions, context: &mut Context) -> RetType;

    /// Shutdown the middleware for a given context.
    ///
    /// The given context must be a valid context which has been initialized with [`Self::init()`].
    ///
    /// If context is zero initialized, then [`ret_types::RET_INVALID_ARGUMENT`] is returned.
    /// If context has been already invalidated ([`Self::shutdown()`] was called on it), then
    /// this function is a no-op and [`ret_types::RET_OK`] is returned.
    fn shutdown(context: &mut Context) -> RetType;

    /// Create a node and return a handle to that node.
    ///
    /// This function can fail, and therefore return `None`, if:
    /// - name is not a valid node name
    /// - namespace is not a valid namespace
    /// - context is not valid i.e. it is zero-initialized, or
    ///   its implementation identifier does not match that of
    ///   this API implementation, or has been invalidated by [`Self::shutdown()`]
    /// - memory allocation fails during node creation
    /// - an unspecified error occurs
    ///
    /// Return node handle, or `None` if there was an error.
    fn create_node(context: &mut Context, name: &str, namespace: &str) -> Option<Node>;

    /// Create a guard condition and return a handle to that guard condition.
    fn create_guard_condition(context: &mut Context) -> GuardCondition;

    /// Create a wait set to store conditions that the middleware can wait on.
    fn create_wait_set(context: &mut Context, max_conditions: usize) -> Option<WaitSet>;
}
