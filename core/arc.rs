// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::mem::transmute;
use super::kinds::{Freeze, Send};
use super::clone::{Clone, DeepClone};
use super::ops::Drop;
use super::atomic::{atomic_fence_acq, atomic_xadd_relaxed, atomic_xsub_rel};

struct ArcBox<T> {
    value: T,
    count: int
}

#[unsafe_no_drop_flag]
pub struct Arc<T> {
    priv ptr: *mut ArcBox<T>
}

impl<T: Send + Freeze> Arc<T> {
    #[inline(always)]
    pub fn new(value: T) -> Arc<T> {
        unsafe {
            Arc::new_unchecked(value)
        }
    }
}

impl<T> Arc<T> {
    pub unsafe fn new_unchecked(value: T) -> Arc<T> {
        Arc{ptr: transmute(~ArcBox{value: value, count: 1})}
    }
}

impl<T> Arc<T> {
    #[inline(always)]
    pub fn borrow<'a>(&'a self) -> &'a T {
        unsafe { &(*self.ptr).value }
    }
}

// Reasoning behind the atomic memory ordering:
// http://www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html

#[unsafe_destructor]
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.ptr != 0 as *mut ArcBox<T> {
            unsafe {
                if atomic_xsub_rel(&mut (*self.ptr).count, 1) == 1 {
                    atomic_fence_acq();
                    let _: ~ArcBox<T> = transmute(self.ptr);
                }
            }
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        unsafe {
            atomic_xadd_relaxed(&mut (*self.ptr).count, 1);
            Arc { ptr: self.ptr }
        }
    }
}

impl<T: DeepClone> DeepClone for Arc<T> {
    fn deep_clone(&self) -> Arc<T> {
        unsafe { Arc::new_unchecked(self.borrow().deep_clone()) }
    }
}
