// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(ctypes, cstack)];

pub mod clone;
pub mod fail;
pub mod intrinsics;
pub mod kinds;
pub mod mem;
pub mod ops;
pub mod option;
pub mod ptr;
pub mod rc;
pub mod slice;

pub mod uint;
pub mod u8;
pub mod u16;
pub mod u32;
pub mod u64;

pub mod int;
pub mod i8;
pub mod i16;
pub mod i32;
pub mod i64;

pub mod f32;
pub mod f64;

#[cfg(libc)]
pub mod heap;
#[cfg(libc)]
pub mod libc;
#[cfg(libc)]
pub mod vec;
