// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Concurrent data structures

use super::container::Container;
use super::clone::Clone;
use super::arc::Arc;
use super::deque::Deque;
use super::priority_queue::PriorityQueue;
use super::mem::transmute;
use super::thread::{Mutex, Cond};
use super::ops::Ord;
use super::option::Option;
use super::hash::HashMap;
use super::ops::Eq;
use super::hash::Hash;

trait GenericQueue<T>: Container {
    fn generic_push(&mut self, item: T);
    fn generic_pop(&mut self) -> Option<T>;
}

impl<T> GenericQueue<T> for Deque<T> {
    fn generic_push(&mut self, item: T) { self.push_back(item) }
    fn generic_pop(&mut self) -> Option<T> { self.pop_front() }
}

impl<T: Ord> GenericQueue<T> for PriorityQueue<T> {
    fn generic_push(&mut self, item: T) { self.push(item) }
    fn generic_pop(&mut self) -> Option<T> { self.pop() }
}

#[no_freeze]
struct QueueBox<T> {
    queue: T,
    mutex: Mutex,
    not_empty: Cond
}

struct QueuePtr<T> {
    ptr: Arc<QueueBox<T>>
}

impl<A, T: GenericQueue<A>> QueuePtr<T> {
    fn new(queue: T) -> QueuePtr<T> {
        unsafe {
            let box = QueueBox { queue: queue, mutex: Mutex::new(), not_empty: Cond::new() };
            QueuePtr { ptr: Arc::new_unchecked(box) }
        }
    }

    fn pop(&self) -> A {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            let mut guard = box.mutex.lock_guard();
            while box.queue.is_empty() {
                box.not_empty.wait_guard(&mut guard)
            }
            box.queue.generic_pop().get()
        }
    }

    fn push(&self, item: A) {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            box.queue.generic_push(item);
            box.mutex.unlock();
            box.not_empty.signal()
        }
    }
}

impl<T> Clone for QueuePtr<T> {
    fn clone(&self) -> QueuePtr<T> {
        QueuePtr { ptr: self.ptr.clone() }
    }
}

/// An unbounded, blocking concurrent queue
pub struct Queue<T> {
    priv ptr: QueuePtr<Deque<T>>
}

impl<T> Queue<T> {
    /// Return a new `Queue` instance
    pub fn new() -> Queue<T> {
        Queue { ptr: QueuePtr::new(Deque::new()) }
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Push a value to the back of the queue
    pub fn push(&self, item: T) {
        self.ptr.push(item)
    }
}

impl<T> Clone for Queue<T> {
    /// Return a shallow copy of the queue
    fn clone(&self) -> Queue<T> {
        Queue { ptr: self.ptr.clone() }
    }
}

/// An unbounded, blocking concurrent priority queue
pub struct BlockingPriorityQueue<T> {
    priv ptr: QueuePtr<PriorityQueue<T>>
}

impl<T: Ord> BlockingPriorityQueue<T> {
    /// Return a new `BlockingPriorityQueue` instance
    pub fn new() -> BlockingPriorityQueue<T> {
        BlockingPriorityQueue { ptr: QueuePtr::new(PriorityQueue::new()) }
    }

    /// Pop the largest value from the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Push a value into the queue
    pub fn push(&self, item: T) {
        self.ptr.push(item)
    }
}

impl<T> Clone for BlockingPriorityQueue<T> {
    /// Return a shallow copy of the queue
    fn clone(&self) -> BlockingPriorityQueue<T> {
        BlockingPriorityQueue { ptr: self.ptr.clone() }
    }
}

#[no_freeze]
struct BoundedQueueBox<T> {
    deque: T,
    mutex: Mutex,
    not_empty: Cond,
    not_full: Cond,
    maximum: uint
}

struct BoundedQueuePtr<T> {
    ptr: Arc<BoundedQueueBox<T>>
}

impl<A, T: GenericQueue<A>> BoundedQueuePtr<T> {
    fn new(maximum: uint, queue: T) -> BoundedQueuePtr<T> {
        unsafe {
            let box = BoundedQueueBox { deque: queue, mutex: Mutex::new(), not_empty: Cond::new(),
                                        not_full: Cond::new(), maximum: maximum };
            BoundedQueuePtr { ptr: Arc::new_unchecked(box) }
        }
    }

    fn pop(&self) -> A {
        unsafe {
            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            while box.deque.is_empty() {
                box.not_empty.wait(&mut box.mutex)
            }
            let item = box.deque.generic_pop().get();
            box.mutex.unlock();
            box.not_full.signal();
            item
        }
    }

    fn push(&self, item: A) {
        unsafe {
            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            while box.deque.len() == box.maximum {
                box.not_full.wait(&mut box.mutex)
            }
            box.deque.generic_push(item);
            box.mutex.unlock();
            box.not_empty.signal()
        }
    }
}

impl<T> Clone for BoundedQueuePtr<T> {
    fn clone(&self) -> BoundedQueuePtr<T> {
        BoundedQueuePtr { ptr: self.ptr.clone() }
    }
}

/// A bounded, blocking concurrent queue
pub struct BoundedQueue<T> {
    priv ptr: BoundedQueuePtr<Deque<T>>
}

impl<T> BoundedQueue<T> {
    /// Return a new `BoundedQueue` instance, holding at most `maximum` elements
    pub fn new(maximum: uint) -> BoundedQueue<T> {
        BoundedQueue { ptr: BoundedQueuePtr::new(maximum, Deque::new()) }
    }

    /// Pop the largest value from the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Push a value to the back of the queue, blocking until the queue is not full
    pub fn push(&self, item: T) {
        self.ptr.push(item)
    }
}

impl<T> Clone for BoundedQueue<T> {
    /// Return a shallow copy of the queue
    fn clone(&self) -> BoundedQueue<T> {
        BoundedQueue { ptr: self.ptr.clone() }
    }
}

/// A bounded, blocking concurrent priority queue
pub struct BoundedPriorityQueue<T> {
    priv ptr: BoundedQueuePtr<PriorityQueue<T>>
}

impl<T: Ord> BoundedPriorityQueue<T> {
    /// Return a new `BoundedPriorityQueue` instance, holding at most `maximum` elements
    pub fn new(maximum: uint) -> BoundedPriorityQueue<T> {
        BoundedPriorityQueue { ptr: BoundedQueuePtr::new(maximum, PriorityQueue::new()) }
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Push a value into the queue, blocking until the queue is not full
    pub fn push(&self, item: T) {
        self.ptr.push(item)
    }
}

impl<T> Clone for BoundedPriorityQueue<T> {
    /// Return a shallow copy of the queue
    fn clone(&self) -> BoundedPriorityQueue<T> {
        BoundedPriorityQueue { ptr: self.ptr.clone() }
    }
}

pub struct ConcurrentHashMap<K, V> {
    priv map: HashMap<K, V>,
    priv mutex: Mutex
}

impl<K: Hash + Eq, V> ConcurrentHashMap<K, V> {
    pub fn with_capacity_and_keys(k0: u64, k1: u64, capacity: uint) -> ConcurrentHashMap<K, V> {
        ConcurrentHashMap {
            map: HashMap::with_capacity_and_keys(k0, k1, capacity),
            mutex: Mutex::new()
        }
    }

    pub fn swap(&mut self, k: K, v: V) -> Option<V> {
        unsafe {
            let _guard = self.mutex.lock_guard();
            self.map.swap(k, v)
        }
    }

    pub fn pop(&mut self, k: &K) -> Option<V> {
        unsafe {
            let _guard = self.mutex.lock_guard();
            self.map.pop(k)
        }
    }
}

impl<K: Hash + Eq, V: Clone> ConcurrentHashMap<K, V> {
    pub fn find(&mut self, k: &K) -> Option<V> {
        unsafe {
            let _guard = self.mutex.lock_guard();
            self.map.find(k).map(|v| v.clone())
        }
    }
}
