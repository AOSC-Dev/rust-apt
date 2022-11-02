//! rust-apt provides bindings to `libapt-pkg`.
//! The goal is to eventually have all of the functionality of `python-apt`
//!
//! The source repository is <https://gitlab.com/volian/rust-apt>
//! For more information please see the readme in the source code.
//!
//! Each module has a `raw` submodule containing c++ bindings to `libapt-pkg`
//!
//! These are safe to use in terms of memory,
//! but may cause segfaults if you do something wrong.
//!
//! If you find a way to segfault without using the `libapt-pkg` bindings
//! directly, please report this as a bug.
//!
//! # Features
//! ###### `worker_sizes`
//! Enables certains fields on the [`Worker`] struct. These extra fields don't
//! work on earlier versions of `libapt-pkg`, so they're excluded by default.
//!
//! The following distros (and likewise any versions before them) have been
//! confirmed to not work with this feature enabled:
//! - Ubuntu 18.04

pub mod cache;
pub mod config;
mod depcache;
pub mod package;
pub mod pkgmanager;
pub mod progress;
pub mod records;
mod resolver;
pub mod tagfile;
pub mod util;

// `cargo check` doesn't seem to be able to tell we're using this in the lib
// documentation above, so we have to add an `#[allow]` macro here.
#[allow(unused_imports)]
use progress::Worker;
