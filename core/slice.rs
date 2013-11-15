// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::mem::transmute;
use core::ptr::{offset, read_ptr, swap_ptr};
use core::fail::abort;

pub struct Slice<T> {
    data: *T,
    len: uint
}

pub unsafe fn unchecked_get<'a, T>(xs: &'a [T], index: uint) -> &'a T {
    let slice: Slice<T> = transmute(xs);
    transmute(offset(slice.data, index as int))
}

pub unsafe fn unchecked_mut_get<'a, T>(xs: &'a mut [T], index: uint) -> &'a mut T {
    let slice: Slice<T> = transmute(xs);
    transmute(offset(slice.data, index as int))
}

pub fn len<T>(xs: &[T]) -> uint {
    unsafe {
        let slice: Slice<T> = transmute(xs);
        slice.len
    }
}

pub fn to_ptr<T>(xs: &[T]) -> *T {
    unsafe {
        let slice: Slice<T> = transmute(xs);
        slice.data
    }
}

pub fn slice<'a, T>(xs: &'a [T], start: uint, end: uint) -> &'a [T] {
    if start > end || end > len(xs) {
        abort()
    }
    unsafe {
        let slice: Slice<T> = transmute(xs);
        let new = Slice {
            data: offset(slice.data, start as int),
            len: (end - start)
        };
        transmute(new)
    }
}

pub fn slice_from<'a, T>(xs: &'a [T], start: uint) -> &'a [T] {
    slice(xs, start, len(xs))
}

pub fn slice_to<'a, T>(xs: &'a [T], end: uint) -> &'a [T] {
    slice(xs, 0, end)
}

pub fn split<'a, T>(xs: &'a [T], mid: uint) -> (&'a [T], &'a [T]) {
    (slice_to(xs, mid), slice_from(xs, mid))
}

pub fn to_mut_ptr<T>(xs: &mut [T]) -> *mut T {
    unsafe {
        let slice: Slice<T> = transmute(xs);
        slice.data as *mut T
    }
}

pub fn mut_slice<'a, T>(xs: &'a mut [T], start: uint, end: uint) -> &'a mut [T] {
    if start > end || end > len(xs) {
        abort()
    }
    unsafe {
        let slice: Slice<T> = transmute(xs);
        let new = Slice {
            data: offset(slice.data, start as int),
            len: (end - start)
        };
        transmute(new)
    }
}

pub fn mut_slice_from<'a, T>(xs: &'a mut [T], start: uint) -> &'a mut [T] {
    let length = len(xs);
    mut_slice(xs, start, length)
}

pub fn mut_slice_to<'a, T>(xs: &'a mut [T], end: uint) -> &'a mut [T] {
    mut_slice(xs, 0, end)
}

pub fn mut_split<'a, T>(xs: &'a mut [T], mid: uint) -> (&'a mut [T], &'a mut [T]) {
    unsafe {
        let ys: &'a mut [T] = read_ptr(&xs);
        (mut_slice_to(xs, mid), mut_slice_from(ys, mid))
    }
}

pub fn swap<T>(xs: &mut [T], a: uint, b: uint) {
    unsafe {
        let x: *mut T = &mut xs[a];
        let y: *mut T = &mut xs[b];
        swap_ptr(x, y);
    }
}

pub unsafe fn unchecked_swap<T>(xs: &mut [T], a: uint, b: uint) {
    let x: *mut T = unchecked_mut_get(xs, a);
    let y: *mut T = unchecked_mut_get(xs, b);
    swap_ptr(x, y);
}
