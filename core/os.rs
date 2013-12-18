// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use c_types::c_int;

mod detail {
    use c_types::c_int;
    extern {
        pub fn exit(status: c_int) -> !;
    }
}

pub fn exit(status: c_int) -> ! {
    unsafe { detail::exit(status) }
}

#[cfg(unix)]
/// Returns the platform-specific value of errno
pub fn errno() -> int {
    extern {
        #[cfg(target_os = "linux")]
        #[cfg(target_os = "android")]
        fn __errno_location() -> *c_int;

        #[link_name = "__error"]
        #[cfg(target_os = "macos")]
        #[cfg(target_os = "freebsd")]
        fn __errno_location() -> *c_int;
    }

    unsafe {
        (*__errno_location()) as int
    }
}

#[cfg(windows)]
/// Returns the platform-specific value of errno
pub fn errno() -> uint {
    #[link_name = "kernel32"]
    extern "system" {
        fn GetLastError() -> u32;
    }

    unsafe {
        GetLastError() as uint
    }
}
