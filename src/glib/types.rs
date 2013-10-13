// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::libc;

pub type gchar = libc::c_char;
pub type gshort = libc::c_short;
pub type glong = libc::c_long;
pub type gint = libc::c_int;
pub type gboolean = gint;

pub type guchar = libc::c_uchar;
pub type gushort = libc::c_ushort;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;

pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;

pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *libc::c_void;
