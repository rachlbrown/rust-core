// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::mem::{forget, uninit};
use super::ops::{Eq, Ord};

mod detail {
    extern "rust-intrinsic" {
        pub fn memcpy32<T>(dst: *mut T, src: *T, count: u32);
        pub fn memcpy64<T>(dst: *mut T, src: *T, count: u64);

        pub fn memmove32<T>(dst: *mut T, src: *T, count: u32);
        pub fn memmove64<T>(dst: *mut T, src: *T, count: u64);

        pub fn memset32<T>(dst: *mut T, val: u8, count: u32);
        pub fn memset64<T>(dst: *mut T, val: u8, count: u64);
    }
}

extern "rust-intrinsic" {
    pub fn offset<T>(dst: *T, offset: int) -> *T;
}

#[inline(always)]
#[cfg(target_word_size = "32")]
pub unsafe fn copy_memory<T>(dst: *mut T, src: *T, count: uint) {
    detail::memmove32(dst, src, count as u32)
}

#[inline(always)]
#[cfg(target_word_size = "64")]
pub unsafe fn copy_memory<T>(dst: *mut T, src: *T, count: uint) {
    detail::memmove64(dst, src, count as u64)
}

#[inline(always)]
#[cfg(target_word_size = "32")]
pub unsafe fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *T, count: uint) {
    detail::memcpy32(dst, src, count as u32)
}

#[inline(always)]
#[cfg(target_word_size = "64")]
pub unsafe fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *T, count: uint) {
    detail::memcpy64(dst, src, count as u64)
}

#[inline(always)]
#[cfg(target_word_size = "32")]
pub unsafe fn set_memory<T>(dst: *mut T, c: u8, count: uint) {
    detail::memset32(dst, c, count as u32)
}

#[inline(always)]
#[cfg(target_word_size = "64")]
pub unsafe fn set_memory<T>(dst: *mut T, c: u8, count: uint) {
    detail::memset64(dst, c, count as u64)
}

#[inline]
pub unsafe fn read_ptr<T>(src: *T) -> T {
    let mut tmp: T = uninit();
    copy_nonoverlapping_memory(&mut tmp, src, 1);
    tmp
}

#[inline]
pub unsafe fn swap_ptr<T>(x: *mut T, y: *mut T) {
    let mut tmp: T = uninit();

    copy_nonoverlapping_memory(&mut tmp, x as *T, 1);
    copy_memory(x, y as *T, 1); // `x` and `y` may overlap
    copy_nonoverlapping_memory(y, &tmp, 1);

    forget(tmp);
}

impl<T> Eq for *T {
    #[inline(always)]
    fn eq(&self, other: &*T) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &*T) -> bool { *self != *other }
}

impl<T> Eq for *mut T {
    #[inline(always)]
    fn eq(&self, other: &*mut T) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &*mut T) -> bool { *self != *other }
}

impl<T> Ord for *T {
    #[inline(always)]
    fn lt(&self, other: &*T) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &*T) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &*T) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &*T) -> bool { *self >= *other }
}

impl<T> Ord for *mut T {
    #[inline(always)]
    fn lt(&self, other: &*mut T) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &*mut T) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &*mut T) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &*mut T) -> bool { *self >= *other }
}
