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

use core::clone::Clone;
use core::weak::Strong;
use core::ignore;
use core::fail::abort;

#[path = "../core/mod.rs"]
mod core;

fn test_live() {
    let x = Strong::new(5);
    let y = x.downgrade();
    if !y.upgrade().is_some() { abort() }
}

fn test_dead() {
    let x = Strong::new(5);
    let y = x.downgrade();
    ignore(x);
    if y.upgrade().is_some() { abort() }
}

#[start]
fn main(_: int, _: **u8) -> int {
    test_live();
    test_dead();
    0
}
