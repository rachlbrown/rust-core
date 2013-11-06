// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[no_std];

use core::thread::Thread;
use core::fail::abort;

#[path = "../core/mod.rs"]
mod core;

static mut a: bool = false;
static mut b: bool = false;

extern "C" fn foo(_: *mut u8) -> *mut u8 {
    unsafe { a = true };
    0 as *mut u8
}

extern "C" fn bar(_: *mut u8) -> *mut u8 {
    unsafe { b = true };
    0 as *mut u8
}

#[start]
fn main(_: int, _: **u8) -> int {
    {
        let _a = Thread::new(foo);
        let _b = Thread::new(bar);
    }
    unsafe {
        if !a || !b {
            abort()
        }
    }
    0
}
