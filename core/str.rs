// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::container::Container;
use super::mem::transmute;

impl<'a> Container for &'a str {
    #[no_split_stack]
    fn len(&self) -> uint {
        let (_, l) : (*u8, uint) = unsafe { transmute(*self) };
        l
    }
}

#[inline(always)]
pub fn as_bytes<'a>(string: &'a str) -> &'a [u8] {
    unsafe { transmute(string) }
}
