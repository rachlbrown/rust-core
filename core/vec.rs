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
use super::fail::out_of_memory;
#[cfg(libc)]
use super::heap::{Heap, free};
use super::ops::Drop;
use super::slice::{Slice, iter, unchecked_get};
use super::ptr::{offset, read_ptr};
use super::uint::mul_with_overflow;
use super::option::{Option, Some, None};
use super::iter::Iterator;

pub struct Vec<T, A> {
    priv len: uint,
    priv cap: uint,
    priv ptr: *mut T,
    priv alloc: A
}

#[cfg(libc)]
impl<T> Vec<T, Heap> {
    #[inline(always)]
    pub fn new() -> Vec<T, Heap> {
        Vec::with_alloc(Heap)
    }

    #[inline(always)]
    pub fn with_capacity(capacity: uint) -> Vec<T, Heap> {
        Vec::with_alloc_capacity(Heap, capacity)
    }
}

// FIXME: broken with non-default allocators until generic destructors are fixed:
// https://github.com/mozilla/rust/issues/4252
impl<T, A: Allocator> Vec<T, A> {
    #[inline(always)]
    pub fn with_alloc(alloc: A) -> Vec<T, A> {
        Vec { len: 0, cap: 0, ptr: 0 as *mut T, alloc: alloc }
    }

    pub fn with_alloc_capacity(mut alloc: A, capacity: uint) -> Vec<T, A> {
        if capacity == 0 {
            Vec::with_alloc(alloc)
        } else {
            let (size, overflow) = mul_with_overflow(capacity, size_of::<T>());
            if overflow {
                out_of_memory();
            }
            let (ptr, _) = unsafe { alloc.alloc(size) };
            Vec { len: 0, cap: capacity, ptr: ptr as *mut T, alloc: alloc }
        }
    }
}

impl<T, A: Allocator> Vec<T, A> {
    #[inline(always)]
    pub fn len(&self) -> uint {
        self.len
    }

    #[inline(always)]
    pub fn capacity(&self) -> uint {
        self.cap
    }

    pub fn reserve(&mut self, capacity: uint) {
        if capacity >= self.len {
            let (size, overflow) = mul_with_overflow(capacity, size_of::<T>());
            if overflow {
                out_of_memory();
            }
            self.cap = capacity;
            unsafe {
                let (ptr, _) = self.alloc.realloc(self.ptr as *mut u8, size);
                self.ptr = ptr as *mut T;
            }
        }
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe {
            if self.len == 0 {
                self.alloc.free(self.ptr as *mut u8);
                self.cap = 0;
                self.ptr = 0 as *mut T;
            } else {
                let (ptr, _) = self.alloc.realloc(self.ptr as *mut u8, self.len * size_of::<T>());
                self.ptr = ptr as *mut T;
                self.cap = self.len;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(read_ptr(unchecked_get(self.as_slice(), self.len())))
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
    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        let slice = Slice { data: self.ptr as *T, len: self.len };
        unsafe { transmute(slice) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        let slice = Slice { data: self.ptr as *T, len: self.len };
        unsafe { transmute(slice) }
    }
}


// FIXME: broken with non-default allocators until generic destructors are fixed:
// https://github.com/mozilla/rust/issues/4252
#[cfg(libc)]
#[unsafe_destructor]
impl<T> Drop for Vec<T, Heap> {
    fn drop(&mut self) {
        unsafe {
            for x in iter(self.as_mut_slice()) {
                read_ptr(x);
            }
            free(self.ptr as *mut u8)
        }
    }
}
