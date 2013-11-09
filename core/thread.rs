// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::platform::c_types::{c_int, pthread_t, pthread_attr_t};
use super::fail::{abort, assert};
use super::ops::Drop;
use super::mem::{uninit, transmute};

extern {
    fn pthread_create(thread: *mut pthread_t, attr: *pthread_attr_t,
                      start_routine: extern "C" fn(*mut u8) -> *mut u8,
                      arg: *mut u8) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut u8) -> c_int;
    fn sched_yield() -> c_int;
}

/// An owned thread type, joined in the destructor.
pub struct Thread<A> {
    priv thread: pthread_t
}

extern "C" fn shim(box: *mut u8) -> *mut u8 {
    let start_routine = unsafe { *transmute::<*mut u8, ~~fn() -> *mut u8>(box) };
    start_routine()
}

// FIXME: should take `proc() -> A`, but `shim` cannot currently be made generic
// https://github.com/mozilla/rust/issues/10353
pub fn spawn<A>(start_routine: proc() -> ~A) -> Thread<A> {
    unsafe {
        let box: *mut u8 = transmute(~start_routine);
        let mut thread = uninit();
        if pthread_create(&mut thread, 0 as *pthread_attr_t, shim, box) != 0 {
            abort()
        }
        Thread { thread: thread }
    }
}

impl<A> Thread<A> {
    pub fn join(self) -> ~A {
        unsafe {
            let mut result = uninit();
            assert(pthread_join(self.thread, &mut result) == 0);
            transmute(result)
        }
    }
}

#[unsafe_destructor]
impl<A> Drop for Thread<A> {
    fn drop(&mut self) {
        unsafe {
            let mut result = uninit();
            assert(pthread_join(self.thread, &mut result) == 0);
            let _: ~A = transmute(result);
        }
    }
}

/// Yield control from the current thread
pub fn deschedule() {
    unsafe {
        assert(sched_yield() == 0)
    }
}