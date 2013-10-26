// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod detail {
    extern "rust-intrinsic" {
        pub fn size_of<T>() -> uint;
        pub fn min_align_of<T>() -> uint;
        pub fn pref_align_of<T>() -> uint;
    }
}

#[inline]
pub fn size_of<T>() -> uint {
    unsafe { detail::size_of::<T>() }
}

#[inline]
pub fn min_align_of<T>() -> uint {
    unsafe { detail::min_align_of::<T>() }
}

#[inline]
pub fn pref_align_of<T>() -> uint {
    unsafe { detail::pref_align_of::<T>() }
}

extern "rust-intrinsic" {
    pub fn forget<T>(_: T) -> ();
    pub fn transmute<T, U>(thing: T) -> U;
    pub fn init<T>() -> T;
    pub fn uninit<T>() -> T;
    pub fn move_val_init<T>(dst: &mut T, src: T);
}

pub trait Allocator {
    /// Allocate at least `size` bytes of memory.
    ///
    /// The `size` parameter must be non-zero. Return a pointer to the memory
    /// allocation and the usable size.
    unsafe fn alloc(&mut self, size: uint) -> (*mut u8, uint);

    /// Change the size of the memory allocation pointed to by `ptr` to `size`.
    ///
    /// The `size` parameter must be non-zero. Return a pointer to the memory
    /// allocation and the usable size.
    unsafe fn realloc(&mut self, ptr: *mut u8, size: uint) -> (*mut u8, uint);

    /// Free the memory allocation pointed to by `ptr`.
    unsafe fn free(&mut self, ptr: *mut u8);
}
