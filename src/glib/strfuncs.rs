// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

/// RAII wrapper of an allocated string, such as one allocated by [g_strdup()](https://developer.gnome.org/glib/stable/glib-String-Utility-Functions.html#g-strdup),
/// which will be freed with [g_free()](https://developer.gnome.org/glib/stable/glib-Memory-Allocation.html#g-free).
pub struct Strdup {
    priv str: *mut ::gchar
}

impl Strdup {
    unsafe fn new(str: *mut ::gchar) -> Strdup {
        Strdup { str: str }
    }

    fn get(&self) -> *::gchar {
        self.str as *::gchar
    }

    fn mut_get(&mut self) -> *mut ::gchar {
        self.str
    }
}

impl Drop for Strdup {
    fn drop(&mut self) {
        unsafe {
            ::detail::mem::g_free(self.str as ::gpointer);
            self.str = ptr::mut_null();
        }
    }
}

/// RAII wrapper of a `NULL`-terminated array of allocated strings, such as allocated by
/// [g_strdupv()](https://developer.gnome.org/glib/stable/glib-String-Utility-Functions.html#g-strdupv),
/// which will be freed with [g_strfreev()](https://developer.gnome.org/glib/stable/glib-String-Utility-Functions.html#g-strfreev).
pub struct Strdupv {
    priv str_array: *mut *mut ::gchar
}

impl Strdupv {
    unsafe fn new(str_array: *mut *mut ::gchar) -> Strdupv {
        Strdupv { str_array: str_array }
    }

    fn mut_get(&mut self) -> *mut *mut ::gchar {
        self.str_array
    }
}

impl Drop for Strdupv {
    fn drop(&mut self) {
        unsafe {
            ::detail::strfuncs::g_strfreev(self.str_array);
            self.str_array = ptr::mut_null();
        }
    }
}

pub unsafe fn strdup(str: *::gchar) -> Strdup {
    Strdup::new(::detail::strfuncs::g_strdup(str))
}

pub unsafe fn strdupv(str_array: *mut *mut ::gchar) -> Strdupv {
    Strdupv::new(::detail::strfuncs::g_strdupv(str_array))
}
