// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "glib", vers = "0.1")];
#[crate_type = "lib"];

use std::libc;

pub use error::Error;
pub use types::*;

pub mod detail;
pub mod error;
pub mod quark;
pub mod strfuncs;
pub mod string;
pub mod types;

pub type gint8 = i8;
pub type guint8 = u8;
pub type gint16 = i16;
pub type guint16 = u16;
pub type gint32 = i32;
pub type guint32 = u32;
pub type gint64 = i64;
pub type guint64 = u64;

pub type gssize = libc::ssize_t;
pub type gsize = libc::size_t;

pub type gintptr = libc::intptr_t;
pub type guintptr = libc::uintptr_t;
