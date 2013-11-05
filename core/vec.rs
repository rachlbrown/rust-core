// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::mem::{Allocator, move_val_init, size_of, transmute};
use super::heap::{Heap, out_of_memory, malloc_raw, free};
use super::kinds::{Freeze, Send};
use super::ops::Drop;
use super::slice::Slice;
use super::ptr::{offset, read_ptr};
use super::uint::mul_with_overflow;

pub struct Vec<T, A> {
    priv len: uint,
    priv cap: uint,
    priv ptr: *mut T,
    priv alloc: A
}

impl<T: Send + Freeze> Vec<T, Heap> {
    #[inline(always)]
    pub fn new() -> Vec<T, Heap> {
        Vec { len: 0, cap: 0, ptr: 0 as *mut T, alloc: Heap }
    }

    pub fn with_capacity(capacity: uint) -> Vec<T, Heap> {
        let (size, overflow) = mul_with_overflow(capacity, size_of::<T>());
        if overflow {
            out_of_memory();
        }
        let ptr = unsafe { malloc_raw(size) };
        Vec { len: 0, cap: capacity, ptr: ptr as *mut T, alloc: Heap }
    }
}

/* FIXME: blocked on the destructor being able to use the allocator (see below)
impl<T: Send + Freeze, A: Allocator> Vec<T, A> {
    #[inline(always)]
    pub fn with_alloc(alloc: A) -> Vec<T, A> {
        Vec { len: 0, cap: 0, ptr: 0 as *mut T, alloc: alloc }
    }

    pub fn with_alloc_capacity(mut alloc: A, capacity: uint) -> Vec<T, A> {
        let (size, overflow) = mul_with_overflow(capacity, size_of::<T>());
        if overflow {
            out_of_memory();
        }
        let (ptr, _) = unsafe { alloc.alloc(size) };
        Vec { len: 0, cap: capacity, ptr: ptr as *mut T, alloc: alloc }
    }
}
*/

impl<T: Send + Freeze, A: Allocator> Vec<T, A> {
    #[inline(always)]
    pub fn len(&self) -> uint {
        self.len
    }

    #[inline(always)]
    pub fn capacity(&self) -> uint {
        self.cap
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe {
            if self.len == 0 {
                self.alloc.free(self.ptr as *mut u8);
                self.cap = 0;
                self.ptr = 0 as *mut T;
            } else {
                let (ptr, _) = self.alloc.realloc(self.ptr as *mut u8, self.cap * size_of::<T>());
                self.ptr = ptr as *mut T;
            }
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        unsafe {
            if self.len == self.cap {
                if self.cap == 0 { self.cap += 2 }
                let old_size = self.cap * size_of::<T>();
                self.cap = self.cap * 2;
                let size = old_size * 2;
                if old_size > size { out_of_memory() }
                let (ptr, _) = self.alloc.realloc(self.ptr as *mut u8, size);
                self.ptr = ptr as *mut T;
            }

            let end = offset(self.ptr as *T, self.len as int) as *mut T;
            move_val_init(&mut *end, value);
            self.len += 1;
        }
    }

    #[inline]
    pub fn as_slice<'r>(&'r self) -> &'r [T] {
        let slice = Slice { data: self.ptr as *T, len: self.len };
        unsafe { transmute(slice) }
    }

    #[inline]
    pub fn as_mut_slice<'r>(&'r mut self) -> &'r mut [T] {
        let slice = Slice { data: self.ptr as *T, len: self.len };
        unsafe { transmute(slice) }
    }
}


// FIXME: use the allocator, blocked on https://github.com/mozilla/rust/issues/4252
#[unsafe_destructor]
impl<T: Send + Freeze> Drop for Vec<T, Heap> {
    fn drop(&mut self) {
        unsafe {
            let mut i = 0;
            let len = self.len();
            let xs = self.as_mut_slice();
            while i < len {
                read_ptr(&xs[i]);
                i += 1;
            }
            free(self.ptr as *mut u8)
        }
    }
}
