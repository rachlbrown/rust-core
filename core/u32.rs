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
    pub fn u32_add_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u32_mul_with_overflow(x: u32, y: u32) -> (u32, bool);
}

pub fn bswap(x: u32) -> u32 {
    super::i32::bswap(x as i32) as u32
}

#[cfg(target = "big")]
pub fn to_be(x: u32) -> u32 {
    x
}

#[cfg(target = "little")]
pub fn to_be(x: u32) -> u32 {
    bswap(x)
}

#[cfg(target = "big")]
pub fn to_le(x: u32) -> u32 {
    bswap(x)
}

#[cfg(target = "little")]
pub fn to_le(x: u32) -> u32 {
    x
}
