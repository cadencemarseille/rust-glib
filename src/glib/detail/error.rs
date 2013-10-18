// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct GError {
    domain: ::detail::quark::GQuark,
    code: ::gint,
    message: *mut ::gchar
}

pub unsafe fn g_error_free(error: *mut GError) {
    #[fixed_stack_segment]; #[inline(never)];
    ::detail::native::error::g_error_free(error)
}
