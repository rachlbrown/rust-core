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
#[feature(macro_rules)];

extern mod core;

use core::fail::abort;
use core::cmp::expect;

#[path = "../macros.rs"]
mod macros;

#[start]
fn main(argc: int, _: **u8) -> int {
    if likely!(argc == 1) {
        0
    } else {
        abort()
    }
}
