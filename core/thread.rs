// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::c_types::{c_int, pthread_t, pthread_attr_t, pthread_mutex_t, pthread_mutex_attr_t};
use super::c_types::{pthread_cond_t, pthread_cond_attr_t};
use super::fail::{abort, assert};
use super::ops::Drop;
use super::mem::{forget, uninit, transmute};

extern {
    fn pthread_create(thread: *mut pthread_t, attr: *pthread_attr_t,
                      start_routine: extern "C" fn(*mut u8) -> *mut u8,
                      arg: *mut u8) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut u8) -> c_int;

    fn sched_yield() -> c_int;

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;

    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *pthread_mutex_attr_t) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;

    fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *pthread_cond_attr_t) -> c_int;
    fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
}

static PTHREAD_CREATE_DETACHED: c_int = 1;
static EBUSY: c_int = 16;

/// An owned thread type, joined in the destructor.
pub struct Thread<A> {
    priv thread: pthread_t
}

extern "C" fn shim(box: *mut u8) -> *mut u8 {
    let start_routine = unsafe { *transmute::<*mut u8, ~proc() -> *mut u8>(box) };
    start_routine()
}

pub fn spawn<A>(start_routine: proc() -> A) -> Thread<A> {
    unsafe {
        // FIXME: this wrapper should be unnecessary, shim should be a generic function instead
        // https://github.com/mozilla/rust/issues/10353
        let wrapper: proc() -> ~A = || ~start_routine();
        let box: *mut u8 = transmute(~wrapper);
        let mut thread = uninit();
        if pthread_create(&mut thread, 0 as *pthread_attr_t, shim, box) != 0 {
            abort()
        }
        Thread { thread: thread }
    }
}

extern "C" fn detached_shim(box: *mut u8) -> *mut u8 {
    let start_routine = unsafe { *transmute::<*mut u8, ~proc()>(box) };
    start_routine();
    0 as *mut u8
}

pub fn spawn_detached(start_routine: proc()) {
    unsafe {
        let box: *mut u8 = transmute(~start_routine);
        let mut attr = uninit();
        if pthread_attr_init(&mut attr) != 0 {
            abort()
        }
        pthread_attr_setdetachstate(&mut attr, PTHREAD_CREATE_DETACHED);
        let mut thread = uninit();
        if pthread_create(&mut thread, &attr, detached_shim, box) != 0 {
            abort()
        }
        assert(pthread_attr_destroy(&mut attr) == 0);
    }
}

impl<A> Thread<A> {
    pub fn join(self) -> ~A {
        unsafe {
            let mut result = uninit();
            assert(pthread_join(self.thread, &mut result) == 0);
            forget(self);
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

pub struct Mutex {
    priv mutex: pthread_mutex_t
}

impl Mutex {
    pub fn new() -> Mutex {
        unsafe {
            let mut mutex = uninit();
            if pthread_mutex_init(&mut mutex, 0 as *pthread_mutex_attr_t) != 0 {
                abort()
            }
            Mutex { mutex: mutex }
        }
    }

    pub unsafe fn lock(&mut self) {
        assert(pthread_mutex_lock(&mut self.mutex) == 0)
    }

    pub unsafe fn lock_guard<'a>(&'a mut self) -> LockGuard<'a> {
        self.lock();
        LockGuard { mutex: self }
    }

    /// Try to grab ownership of a lock, and return `true` if successful
    pub unsafe fn trylock(&mut self) -> bool {
        let rc = pthread_mutex_trylock(&mut self.mutex);
        if rc == EBUSY {
            false
        } else {
            assert(rc == 0);
            true
        }
    }

    pub unsafe fn unlock(&mut self) {
        assert(pthread_mutex_unlock(&mut self.mutex) == 0)
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe {
            assert(pthread_mutex_destroy(&mut self.mutex) == 0)
        }
    }
}

pub struct Cond {
    priv cond: pthread_cond_t
}

impl Cond {
    pub fn new() -> Cond {
        unsafe {
            let mut cond = uninit();
            if pthread_cond_init(&mut cond, 0 as *pthread_cond_attr_t) != 0 {
                abort()
            }
            Cond { cond: cond }
        }
    }

    pub unsafe fn signal(&mut self) {
        assert(pthread_cond_signal(&mut self.cond) == 0)
    }

    pub unsafe fn broadcast(&mut self) {
        assert(pthread_cond_broadcast(&mut self.cond) == 0)
    }

    pub unsafe fn wait(&mut self, mutex: &mut Mutex) {
        assert(pthread_cond_wait(&mut self.cond, &mut mutex.mutex) == 0)
    }

    pub unsafe fn wait_guard(&mut self, guard: &mut LockGuard) {
        self.wait(guard.mutex)
    }
}

impl Drop for Cond {
    fn drop(&mut self) {
        unsafe {
            assert(pthread_cond_destroy(&mut self.cond) == 0);
        }
    }
}

pub struct LockGuard<'a> {
    priv mutex: &'a mut Mutex
}

#[unsafe_destructor]
impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            self.mutex.unlock()
        }
    }
}
