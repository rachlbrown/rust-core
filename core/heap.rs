// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use fail::out_of_memory;

mod detail {
    extern {
        pub fn free(ptr: *mut u8);
    }
}

extern {
    pub fn calloc(nmemb: uint, size: uint) -> *mut u8;
    pub fn malloc(size: uint) -> *mut u8;
    pub fn realloc(ptr: *mut u8, size: uint) -> *mut u8;
    pub fn aligned_alloc(align: uint, size: uint) -> *mut u8;
}

#[inline(always)]
#[lang = "exchange_free"]
pub unsafe fn free(ptr: *mut u8) {
    detail::free(ptr)
}

#[inline]
#[lang = "exchange_malloc"]
pub unsafe fn malloc_raw(size: uint) -> *mut u8 {
    let ptr = malloc(size);
    if ptr == 0 as *mut u8 {
        out_of_memory()
    }
    ptr
}

pub unsafe fn calloc_raw(count: uint, size: uint) -> *mut u8 {
    let ptr = calloc(count, size);
    if ptr == 0 as *mut u8 {
        out_of_memory()
    }
    ptr
}

#[inline]
pub unsafe fn aligned_alloc_raw(align: uint, size: uint) -> *mut u8 {
    let ptr = aligned_alloc(align, size);
    if ptr == 0 as *mut u8 {
        out_of_memory()
    }
    ptr
}

#[inline]
pub unsafe fn realloc_raw(ptr: *mut u8, size: uint) -> *mut u8 {
    let ptr = realloc(ptr, size);
    if ptr == 0 as *mut u8 {
        out_of_memory()
    }
    ptr
}
