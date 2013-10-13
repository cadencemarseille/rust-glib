// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

/// RAII wrapper of an allocated string, which will be freed with `g_free()`.
pub struct Strdup {
    str: *mut ::gchar
}

impl Strdup {
    unsafe fn new(str: *mut ::gchar) -> Strdup {
        Strdup { str: str }
    }
}

impl Drop for Strdup {
    fn drop(&mut self) {
        ::detail::mem::g_free(self.str as ::gpointer);
        self.str = ptr::mut_null();
    }
}

/// RAII wrapper of a `NULL`-terminated array of allocated strings, which will be freed
/// with `g_strdupv()`.
pub struct Strdupv {
    str_array: *mut *mut ::gchar
}

impl Strdupv {
    unsafe fn new(str_array: *mut *mut ::gchar) -> Strdupv {
        Strdupv { str_array: str_array }
    }
}

impl Drop for Strdupv {
    fn drop(&mut self) {
        ::detail::strfuncs::g_strfreev(self.str_array);
        self.str_array = ptr::mut_null();
    }
}

fn strdup(str: *::gchar) -> Strdup {
    Strdup { str: ::detail::strfuncs::g_strdup(str) }
}

fn strdupv(str_array: *mut *mut ::gchar) -> Strdupv {
    Strdupv { str_array: ::detail::strfuncs::g_strdupv(str_array) }
}
