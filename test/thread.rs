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

use core::thread::spawn;
use core::fail::abort;

#[path = "../core/mod.rs"]
mod core;

fn foo() -> int {
    10
}

fn bar() -> int {
    5
}

fn baz() {}

#[start]
fn main(_: int, _: **u8) -> int {
    let a = spawn(foo);
    let b = spawn(bar);
    let _c = spawn(baz);

    if *a.join() != 10 || *b.join() != 5 {
        abort()
    }

    0
}
