// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::c_str::ToCStr;
use std::container::Container;
use std::ptr;

pub struct String {
    priv ptr: *::detail::string::GString
}

impl String {
    fn new<'a, S: Container + ToCStr>(init: &'a S) -> String {
        unsafe {
            do init.with_c_str_unchecked |init_c_str| -> String {
                String {
                    ptr: ::detail::string::g_string_new_len(init_c_str, init.len() as ::gssize) as *::detail::string::GString
                }
            }
        }
    }
}

impl Clone for String {
    fn clone(&self) -> String {
        let new_ptr = {
            if ptr::is_null(self.ptr) {
                ptr::null()
            } else {
                unsafe { ::detail::string::g_string_new_len((*self.ptr).str as *::gchar, (*self.ptr).len as ::gssize) as *::detail::string::GString }
            }
        };
        String {
            ptr: new_ptr
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        ::detail::string::g_string_free(self.ptr as *mut ::detail::string::GString, false);
        self.ptr = ptr::null();
    }
}

impl Eq for String {
    fn eq(&self, other: &String) -> bool {
        ::detail::string::g_string_equal(self.ptr, other.ptr)
    }
}
