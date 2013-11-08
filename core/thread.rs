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
use super::mem::uninit;

extern {
    fn pthread_create(thread: *mut pthread_t, attr: *pthread_attr_t,
                      start_routine: extern "C" fn(*mut u8) -> *mut u8,
                      arg: *mut u8) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut u8) -> c_int;
    fn sched_yield() -> c_int;
}

pub struct Thread {
    priv thread: pthread_t
}

impl Thread {
    pub fn new(start_routine: extern "C" fn(arg: *mut u8) -> *mut u8) -> Thread {
        unsafe {
            let mut thread = uninit();
            if pthread_create(&mut thread, 0 as *pthread_attr_t, start_routine, 0 as *mut u8) != 0 {
                abort()
            }
            Thread { thread: thread }
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            assert(pthread_join(self.thread, 0 as *mut *mut u8) == 0);
        }
    }
}

pub fn deschedule() {
    unsafe {
        assert(sched_yield() == 0)
    }
}
