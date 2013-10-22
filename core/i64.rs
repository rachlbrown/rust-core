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
    fn bswap64(x: i64) -> i64;
    pub fn ctpop64(x: i64) -> i64;
    pub fn ctlz64(x: i64) -> i64;
    pub fn cttz64(x: i64) -> i64;
    pub fn i64_add_with_overflow(x: i64, y: i64) -> (i64, bool);
    pub fn i64_sub_with_overflow(x: i64, y: i64) -> (i64, bool);
    pub fn i64_mul_with_overflow(x: i64, y: i64) -> (i64, bool);
}

pub fn bswap(x: i64) -> i64 {
    unsafe { bswap64(x) }
}

#[cfg(target = "big")]
pub fn to_be(x: i64) -> i64 {
    x
}

#[cfg(target = "little")]
pub fn to_be(x: i64) -> i64 {
    bswap(x)
}

#[cfg(target = "big")]
pub fn to_le(x: i64) -> i64 {
    bswap(x)
}

#[cfg(target = "little")]
pub fn to_le(x: i64) -> i64 {
    x
}
