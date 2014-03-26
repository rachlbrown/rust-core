// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use container::Container;
use cmp::Eq;
use mem::transmute;

impl<'a> Container for &'a str {
    #[no_split_stack]
    fn len(&self) -> uint {
        let (_, l) : (*u8, uint) = unsafe { transmute(*self) };
        l
    }
}

impl<'a> Eq for &'a str {
	fn eq(&self, other: & &'a str) -> bool {
		unsafe { eq_slice(*self, *other) }
	}
}

#[inline(always)]
pub fn as_bytes<'a>(string: &'a str) -> &'a [u8] {
    unsafe { transmute(string) }
}

unsafe fn memcmp (a: *u8, b: *u8, len: uint) -> bool {
	let mut x = 0;
	let a = a as uint;
	let b = b as uint;
	while x < len {
			if (*((a + x) as *u8) != *((b + x) as *u8)) { return false; }
			x += 1;
		}
	true
}

#[lang="str_eq"]
#[inline]
pub unsafe fn eq_slice(a: &str, b:&str) ->bool {
	let (amem, al) : (*u8, uint) = unsafe { transmute(a) };
	let (bmem, bl) : (*u8, uint) = unsafe { transmute(b) };
	
	al == bl && memcmp(amem, bmem, al)
}
