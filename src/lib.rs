//! <p style="margin: -10px 0 0 15px; padding: 0; float: right;">
//!   <a href="https://sentry.io/"><img
//!     src="https://sentry-brand.storage.googleapis.com/sentry-logo-black.png"
//!     style="width: 260px"></a>
//! </p>
//!
//! This crate provides support for logging events and errors / panics to
//! the [Sentry](https://sentry.io/) error logging service.  It integrates with
//! the standard panic system in Rust as well as a few popular error handling
//! setups.
//!
//! # Quickstart
//!
//! To use the crate you need to create a client first.  When a client is created
//! it's typically bound to the current thread by calling `bind_client`.  By default
//! this happens by using the `sentry::init` convenience function.  When the client
//! is bound to the main thread it also becomes the default client for future
//! threads created but it is always possible to override the client for a thread
//! later by explicitly binding it.
//!
//! The `sentry::init` function returns a guard that when dropped will flush
//! Events that were not yet sent to the sentry service.  It has a two second
//! deadline for this so shutdown of applications might slightly delay as a result
//! of this.
//!
//! ```
//! extern crate sentry;
//!
//! fn main() {
//!     let _guard = sentry::init("https://key@sentry.io/42");
//!     sentry::capture_message("Hello World!", sentry::Level::Info);
//!     // when the guard goes out of scope here, the client will wait up to two
//!     // seconds to send remaining events to the service.
//! }
//! ```
//!
//! # Integrations
//!
//! What makes this crate useful are the various integrations that exist.  Some
//! of them are enabled by default, some uncommon ones or for deprecated parts of
//! the ecosystem a feature flag needs to be enabled.  For the available
//! integrations and how to use them see [integrations](integrations/index.html).
//!
//! # Shim Only API
//!
//! This crate can also be used in "shim only" mode.  This is enabled by disabling all
//! default features of the crate.  In that mode a minimal API set is retained that
//! can be used to instrument code for Sentry without actually using Sentry.  The
//! shim is a small set of APIs that dispatch to the underlying implementations on
//! the configured Sentry client.  If the client is not there the shim will blackhole
//! a lot of operations.
//!
//! Only if a user then also uses and configures Sentry this code becomes used.
//!
//! In shim mode some types are restricted in functionality.  For instance the `Client` in shim
//! mode does not retain all API functionality.  To see what the APIs in shim-only
//! mode look like refer to [the shim only docs](shim/index.html).
#[warn(missing_docs)]
#[cfg(feature = "with_client_implementation")]
extern crate backtrace;
#[cfg(feature = "with_client_implementation")]
extern crate im;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "with_client_implementation")]
extern crate reqwest;
extern crate sentry_types;
extern crate serde;
extern crate serde_json;
#[cfg(feature = "with_client_implementation")]
extern crate url;
extern crate uuid;

#[cfg(feature = "with_device_info")]
extern crate libc;

#[cfg(feature = "with_device_info")]
extern crate hostname;

#[cfg(all(feature = "with_device_info", not(windows)))]
extern crate uname;

#[cfg(feature = "with_failure")]
extern crate regex;

#[cfg(feature = "with_failure")]
extern crate failure;

#[cfg(feature = "with_error_chain")]
extern crate error_chain;

#[cfg(feature = "with_log")]
extern crate log;

#[cfg(feature = "with_debug_meta")]
extern crate findshlibs;

#[macro_use]
mod macros;

mod client;
mod scope;
mod api;

#[cfg(feature = "with_client_implementation")]
mod constants;
#[cfg(feature = "with_client_implementation")]
mod transport;
#[cfg(feature = "with_client_implementation")]
pub mod utils;
#[cfg(feature = "with_client_implementation")]
pub mod integrations;
#[cfg(feature = "with_client_implementation")]
mod backtrace_support;

/// The shim only API.
///
/// This module does not exist normally but it's typically compiled for documentation
/// purposes so that users can see the API subset trivially that is available for
/// environments where all features are disabled.
#[cfg(feature = "with_shim_api")]
pub mod shim {
    pub use client::shim::*;
    pub use scope::shim::*;
}

pub use api::*;
