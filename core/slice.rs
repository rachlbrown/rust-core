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

pub fn len<T>(xs: &[T]) -> uint {
    unsafe {
        let slice: Slice<T> = transmute(xs);
        slice.len
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
