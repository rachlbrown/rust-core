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
    pub fn u16_add_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u16_sub_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u16_mul_with_overflow(x: u16, y: u16) -> (u16, bool);
}

pub fn bswap(x: u16) -> u16 {
    super::i16::bswap(x as i16) as u16
}

#[cfg(target = "big")]
pub fn to_be(x: u16) -> u16 {
    x
}

#[cfg(target = "little")]
pub fn to_be(x: u16) -> u16 {
    bswap(x)
}

#[cfg(target = "big")]
pub fn to_le(x: u16) -> u16 {
    bswap(x)
}

#[cfg(target = "little")]
pub fn to_le(x: u16) -> u16 {
    x
}
