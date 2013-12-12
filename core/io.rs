// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use container::Container;
use c_types::c_int;
use slice::{to_mut_ptr, to_ptr};

enum FILE {}

mod detail {
    use super::FILE;

    extern {
        pub static stdin: *mut FILE;
        pub static stdout: *mut FILE;
        pub static stderr: *mut FILE;
    }
}

extern {
    fn fread(ptr: *mut u8, size: uint, nmemb: uint, stream: *mut FILE) -> uint;
    fn fwrite(ptr: *u8, size: uint, nmemb: uint, stream: *mut FILE) -> uint;
    fn fflush(fp: *mut FILE) -> c_int;
}

pub struct StdStream {
    priv file: *mut FILE
}

pub fn stdin() -> StdStream {
    StdStream { file: detail::stdin }
}

pub fn stdout() -> StdStream {
    StdStream { file: detail::stdout }
}

pub fn stderr() -> StdStream {
    StdStream { file: detail::stderr }
}

impl StdStream {
    pub fn read(&mut self, xs: &mut [u8]) -> uint {
        unsafe {
            fread(to_mut_ptr(xs), 1, xs.len(), self.file)
        }
    }

    pub fn write(&mut self, xs: &[u8]) -> uint {
        unsafe {
            fwrite(to_ptr(xs), 1, xs.len(), self.file)
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            fflush(self.file);
        }
    }
}
