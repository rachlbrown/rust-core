// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn bswap(x: u64) -> u64 {
    super::i64::bswap(x as i64) as u64
}

#[cfg(target = "big")]
pub fn to_be(x: u64) -> u64 {
    x
}

#[cfg(target = "little")]
pub fn to_be(x: u64) -> u64 {
    bswap(x)
}

#[cfg(target = "big")]
pub fn to_le(x: u64) -> u64 {
    bswap(x)
}

#[cfg(target = "little")]
pub fn to_le(x: u64) -> u64 {
    x
}
