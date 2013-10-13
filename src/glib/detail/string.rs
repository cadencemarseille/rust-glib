// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct GString {
    str: *mut ::gchar,
    len: ::gsize,
    allocated_len: ::gsize
}

pub fn g_string_equal(v: *::detail::string::GString, v2: *::detail::string::GString) -> bool {
    #[fixed_stack_segment]; #[inline(never)];
    let res = unsafe { ::detail::native::string::g_string_equal(v, v2) };
    if res == 0 { false } else { true }
}

pub fn g_string_free(string: *mut ::detail::string::GString, free_segment: bool) -> *mut ::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe { ::detail::native::string::g_string_free(string, if free_segment { 1 } else { 0 }) }
}

pub fn g_string_new_len(init: *::gchar, len: ::gssize) -> *mut ::detail::string::GString {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe { ::detail::native::string::g_string_new_len(init, len) }
}
