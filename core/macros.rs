// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_escape]

#[macro_export]
macro_rules! likely(
    ($val:expr) => {
        {
            let x: bool = $val;
            unsafe { expect(x as u8, 1) != 0 }
        }
    }
)

#[macro_export]
macro_rules! unlikely(
    ($val:expr) => {
        {
            let x: bool = $val;
            unsafe { expect(x as u8, 0) != 0 }
        }
    }
)

#[macro_export]
macro_rules! thread_local(
    ($name:ident, $t:ty, $init:expr) => {
        mod $name {
            #[thread_local]
            pub static mut VALUE: $t = $init;

            #[inline(always)]
            pub fn set(value: $t) {
                unsafe {
                    VALUE = value;
                }
            }

            #[inline(always)]
            pub fn get() -> $t {
                unsafe {
                    VALUE
                }
            }
        }
    }
)
