// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern "rust-intrinsic" {
    pub fn ctpop8(x: i8) -> i8;
    pub fn ctlz8(x: i8) -> i8;
    pub fn cttz8(x: i8) -> i8;
    pub fn i8_add_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i8_sub_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i8_mul_with_overflow(x: i8, y: i8) -> (i8, bool);
}
