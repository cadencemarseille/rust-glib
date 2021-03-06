// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

pub unsafe fn g_strdup(str: *::gchar) -> *mut ::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    assert!(ptr::is_not_null(str));
    ::detail::native::strfuncs::g_strdup(str)
}

pub unsafe fn g_strdupv(str_array: *mut *mut ::gchar) -> *mut *mut ::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    assert!(ptr::is_not_null(str_array));
    ::detail::native::strfuncs::g_strdupv(str_array)
}

pub unsafe fn g_strfreev(str_array: *mut *mut ::gchar) {
    #[fixed_stack_segment]; #[inline(never)];
    ::detail::native::strfuncs::g_strfreev(str_array)
}
