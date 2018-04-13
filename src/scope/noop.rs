use std::fmt;
use std::sync::Arc;

use api::protocol::{User, Context, Value};
use client::noop::Client;

/// The "shim only" scope.
///
/// In shim only mode all modification functions are available as normally
/// just that generally calling them is impossible.
#[derive(Debug, Clone)]
pub struct Scope;

/// Invokes a function if the sentry client is available with client and scope.
///
/// In shim only mode the closure is never actually executed.
pub fn with_client_and_scope<F, R>(f: F) -> R
where
    F: FnOnce(Arc<Client>, &Scope) -> R,
    R: Default,
{
    let _f = f;
    Default::default()
}

/// A "shim only" scope guard.
///
/// Doesn't do anything but can be debug formatted.
#[derive(Default)]
pub struct ScopeGuard;

impl fmt::Debug for ScopeGuard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScopeGuard")
    }
}

/// Pushes a new scope on the stack.
///
/// A "shim only" scope guard is a zero sized type that doesn't do anything.
#[inline(always)]
pub fn push_scope() -> ScopeGuard {
    ScopeGuard
}

/// Never returns a client.
///
/// In normal situations this would return the client but in shim-only mode
/// this will always return `None`.
pub fn current_client() -> Option<Arc<Client>> {
    None
}

/// Binds a client.
///
/// As its impossible to construct a client in shim only mode this function
/// cannot actually ever be called (it will panic).  The reason this is exposed
/// API in shimmed mode is mostly to propage a client into another thread or
/// similar.
pub fn bind_client(client: Arc<Client>) {
    let _client = client;
    shim_unreachable!();
}

impl Scope {
    pub fn clear(&mut self) {
        shim_unreachable!();
    }

    pub fn set_fingerprint(&mut self, fingerprint: Option<&[&str]>) {
        let _fingerprint = fingerprint;
        shim_unreachable!();
    }

    pub fn set_user(&mut self, user: Option<User>) {
        let _user = user;
        shim_unreachable!();
    }

    pub fn set_tag<V: ToString>(&mut self, key: &str, value: V) {
        let _key = key;
        let _value = value;
        shim_unreachable!();
    }

    pub fn remove_tag(&mut self, key: &str) {
        let _key = key;
        shim_unreachable!();
    }

    pub fn set_context<C: Into<Context>>(&mut self, key: &str, value: C) {
        let _key = key;
        let _value = value;
        shim_unreachable!();
    }

    pub fn remove_context(&mut self, key: &str) {
        let _key = key;
        shim_unreachable!();
    }

    pub fn set_extra(&mut self, key: &str, value: Value) {
        let _key = key;
        let _value = value;
        shim_unreachable!();
    }

    pub fn remove_extra(&mut self, key: &str) {
        let _key = key;
        shim_unreachable!();
    }
}
