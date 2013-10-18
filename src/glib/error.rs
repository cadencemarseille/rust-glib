// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use std::str;

pub struct Error {
    priv ptr: *mut ::detail::error::GError
}

impl Error {
    pub unsafe fn new(ptr: *mut ::detail::error::GError) -> Error {
        assert!(ptr::is_not_null(ptr));
        Error { ptr: ptr }
    }

    pub fn message(&self) -> ~str {
        unsafe {
            assert!(ptr::is_not_null(self.ptr));
            str::raw::from_c_str((*self.ptr).message as *::gchar)
        }
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            assert!(ptr::is_not_null(self.ptr));
            ::detail::error::g_error_free(self.ptr);
            self.ptr = ptr::mut_null();
        }
    }
}
