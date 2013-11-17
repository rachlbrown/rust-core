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
use super::deque::Deque;
use super::mem::transmute;
use super::thread::{Mutex, Cond};

#[no_freeze]
struct QueueBox<T> {
    deque: Deque<T>,
    mutex: Mutex,
    cond: Cond
}

pub struct Queue<T> {
    priv ptr: Arc<QueueBox<T>>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        unsafe {
            let box = QueueBox { deque: Deque::new(), mutex: Mutex::new(), cond: Cond::new() };
            Queue { ptr: Arc::new_unchecked(box) }
        }
    }

    pub fn pop(&self) -> T {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            while box.deque.len() == 0 {
                box.cond.wait(&mut box.mutex)
            }
            let item = box.deque.pop_front().get();
            box.mutex.unlock();
            item
        }
    }

    pub fn push(&self, item: T) {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            box.deque.push_back(item);
            box.mutex.unlock();
            box.cond.signal()
        }
    }
}

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Queue<T> {
        Queue { ptr: self.ptr.clone() }
    }
}
