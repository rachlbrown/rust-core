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
    fn bswap16(x: i16) -> i16;
}

pub fn bswap(x: i16) -> i16 {
    unsafe { bswap16(x) }
}

#[cfg(target = "big")]
pub fn to_be(x: i16) -> i16 {
    x
}

#[cfg(target = "little")]
pub fn to_be(x: i16) -> i16 {
    bswap(x)
}

#[cfg(target = "big")]
pub fn to_le(x: i16) -> i16 {
    bswap(x)
}

#[cfg(target = "little")]
pub fn to_le(x: i16) -> i16 {
    x
}
