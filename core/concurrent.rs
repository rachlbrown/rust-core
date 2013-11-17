// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::clone::Clone;
use super::arc::Arc;
use super::fail::{abort, assert};
use super::ops::Drop;
use super::deque::Deque;
use super::mem::{transmute, uninit};
use super::platform::c_types::{c_int, pthread_mutex_t, pthread_mutex_attr_t};
use super::platform::c_types::{pthread_cond_t, pthread_cond_attr_t};

extern {
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *pthread_mutex_attr_t) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;

    fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *pthread_cond_attr_t) -> c_int;
    fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
}

#[no_freeze]
struct QueueBox<T> {
    priv deque: Deque<T>,
    priv mutex: pthread_mutex_t,
    priv cond: pthread_cond_t
}

pub struct Queue<T> {
    priv ptr: Arc<QueueBox<T>>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        unsafe {
            let mut mutex = uninit();
            let mut cond = uninit();
            if pthread_mutex_init(&mut mutex, 0 as *pthread_mutex_attr_t) != 0 {
                abort()
            }
            if pthread_cond_init(&mut cond, 0 as *pthread_cond_attr_t) != 0 {
                abort()
            }
            let box = QueueBox { deque: Deque::new(), mutex: mutex, cond: cond };
            Queue { ptr: Arc::new_unchecked(box) }
        }
    }

    pub fn pop(&self) -> T {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            assert(pthread_mutex_lock(&mut box.mutex) == 0);
            while box.deque.len() == 0 {
                assert(pthread_cond_wait(&mut box.cond, &mut box.mutex) == 0)
            }
            let item = box.deque.pop_front().get();
            assert(pthread_mutex_unlock(&mut box.mutex) == 0);
            item
        }
    }

    pub fn push(&self, item: T) {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            assert(pthread_mutex_lock(&mut box.mutex) == 0);
            box.deque.push_back(item);
            assert(pthread_mutex_unlock(&mut box.mutex) == 0);
            assert(pthread_cond_signal(&mut box.cond) == 0)
        }
    }
}

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Queue<T> {
        Queue { ptr: self.ptr.clone() }
    }
}

#[unsafe_destructor]
impl<T> Drop for QueueBox<T> {
    fn drop(&mut self) {
        unsafe {
            assert(pthread_mutex_destroy(&mut self.mutex) == 0);
            assert(pthread_cond_destroy(&mut self.cond) == 0)
        }
    }
}
