// Copyright 2013 The rust-glib authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod extra;
extern mod rustc;
extern mod rustpkg;

use rustpkg::api;
use std::os;
use std::run;
use std::str;
use std::util;
use std::vec;

fn cd(path: &Path) {
    if !os::change_dir(path) {
        fail2!("Package script error: Failed to `cd` into `{}`", path.to_str());
    }
}

fn do_install(args: ~[~str]) {
    let sysroot_arg = args[1].clone();
    let sysroot_path = Path(sysroot_arg);

    let glib_libs = match os::getenv("GLIB_LIBS") {
        None => {
            let pkg_config_output = run::process_output("pkg-config", [~"--libs", ~"glib-2.0"]);
            if pkg_config_output.status != 0 {
                fail!("Package script error: `pkg-config --libs glib-2.0` failed");
            }
            let output_ptr = vec::raw::to_ptr(pkg_config_output.output);
            let output_len = pkg_config_output.output.len();
            let libs_str = unsafe { str::raw::from_buf_len(output_ptr, output_len) };
            os::setenv("GLIB_LIBS", libs_str);
            libs_str
        },
        Some(glib_libs) => glib_libs
    };
    // `pkg-config` adds a newline to the end, which we need to trim away because newlines
    // in link_args cause build issues.
    let trimmed_glib_libs = glib_libs.trim();
    debug!("GLIB_LIBS=\"%s\"", trimmed_glib_libs);

    let workspace_path = os::getcwd();

    api::build_lib(sysroot_path, workspace_path, ~"glib", rustpkg::version::ExactRevision(~"0.1"), Path("mod.rs"));
}

fn do_configs(args: ~[~str]) {
    util::ignore(args);
}

fn main() {
    let args = os::args();
    let args_len = args.len();

    if args_len < 2 {
        fail!("Package script requires a directory where rustc libraries live as the first argument");
    } else if args_len < 3 {
        fail!("Package script requires a command as the second argument");
    }

    if args[2] == ~"install" {
        do_install(args);
    } else if args[2] == ~"configs" {
        do_configs(args);
    } else {
        fail2!("Package script error: Unsupported command `{}`", args[2]);
    }
}
