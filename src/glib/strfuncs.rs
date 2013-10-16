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
    /// Creates a new `Strdup` that takes ownership of the given allocated string. The `Strdup`
    /// will free the string via g_free() when destroyed.
    ///
    /// This static method is unsafe because the given string must have been allocated (such
    /// as by g_strdup()) and it must be that only the `Strdup` will free the memory.
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
    /// Creates a `Strdupv` that takes ownership of the given `NULL`-terminated array of
    /// allocated strings, as well as the allocated strings themselves. The `Strdupv` will
    /// free each string up to the `NULL` terminator and free the array (via g_strfreev())
    /// when destroyed.
    ///
    /// This static method is unsafe because (1) the given array must have been allocated;
    /// (2) the given array must be `NULL` terminated; (3) each string in the array up to
    /// the `NULL` terminator must have been allocated; and (4) it must be that only the
    /// `Strdupv` will free the array and each of the strings up to the `NULL` terminator.
    unsafe fn new(str_array: *mut *mut ::gchar) -> Strdupv {
        Strdupv { str_array: str_array }
    }

    /// Gets a mutable pointer to the wrapped array of strings.
    ///
    /// This method is unsafe because it is possible to overwrite the `NULL` terminator,
    /// which could cause a segfault when this `Strdupv` is destroyed.
    unsafe fn mut_get(&mut self) -> *mut *mut ::gchar {
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

/// Duplicates a nul-terminated string.
///
/// This function is unsafe because the given string must be terminated with a nul character
/// (`'\0'`).
pub unsafe fn strdup(str: *::gchar) -> Strdup {
    Strdup::new(::detail::strfuncs::g_strdup(str))
}

/// Duplicates a `NULL`-terminated array of nul-terminated strings.
///
/// This function is unsafe because the given array of strings must be `NULL`-terminated
/// and each string up to the `NULL` terminator must be terminated with a nul character
/// (`'\0'`).
pub unsafe fn strdupv(str_array: *mut *mut ::gchar) -> Strdupv {
    Strdupv::new(::detail::strfuncs::g_strdupv(str_array))
}
