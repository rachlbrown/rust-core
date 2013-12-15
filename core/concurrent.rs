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
//!
//! This module defines mutable concurrent data structures built on top of
//! atomic reference counting (`core::arc`).

use container::Container;
use clone::Clone;
use arc::Arc;
use deque::Deque;
use priority_queue::PriorityQueue;
use mem::{transmute, uninit};
use thread::{Mutex, Cond, Timeout};
use cmp::{Eq, Ord};
use option::{Some, None, Option};
use hash::{Hash, HashMap};
use heap::Heap;
use vec::Vec;
use kinds::Send;
use fail::abort;
use c_types::{c_int, clockid_t, timespec};

static CLOCK_MONOTONIC: clockid_t = 1;

extern {
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
}

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

impl<A: Send, T: GenericQueue<A>> QueuePtr<T> {
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

    fn pop_timeout(&self, reltime: timespec) -> Option<A> {
        unsafe {
            let mut abstime = uninit();
            if clock_gettime(CLOCK_MONOTONIC, &mut abstime) != 0 {
                abort()
            }
            abstime.tv_sec += reltime.tv_sec;
            abstime.tv_nsec += reltime.tv_nsec;

            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            let mut guard = box.mutex.lock_guard();
            while box.queue.is_empty() {
                if box.not_empty.wait_until_guard(&mut guard, abstime) == Timeout {
                    return None
                }
            }
            Some(box.queue.generic_pop().get())
        }
    }

    fn push(&self, item: A) {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            let _guard = box.mutex.lock_guard();
            box.queue.generic_push(item);
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

impl<T: Send> Queue<T> {
    /// Return a new `Queue` instance.
    pub fn new() -> Queue<T> {
        Queue { ptr: QueuePtr::new(Deque::new()) }
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty.
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty or the
    /// timeout expires.
    pub fn pop_timeout(&self, reltime: timespec) -> Option<T> {
        self.ptr.pop_timeout(reltime)
    }

    /// Push a value to the back of the queue.
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

impl<T: Ord + Send> BlockingPriorityQueue<T> {
    /// Return a new `BlockingPriorityQueue` instance.
    pub fn new() -> BlockingPriorityQueue<T> {
        BlockingPriorityQueue { ptr: QueuePtr::new(PriorityQueue::new()) }
    }

    /// Pop the largest value from the queue, blocking until the queue is not empty.
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Pop the largest value from the queue, blocking until the queue is not empty or the timeout
    /// expires.
    pub fn pop_timeout(&self, reltime: timespec) -> Option<T> {
        self.ptr.pop_timeout(reltime)
    }

    /// Push a value into the queue.
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

impl<A: Send, T: GenericQueue<A>> BoundedQueuePtr<T> {
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
            let mut guard = box.mutex.lock_guard();
            while box.deque.is_empty() {
                box.not_empty.wait_guard(&mut guard)
            }
            let item = box.deque.generic_pop().get();
            box.not_full.signal();
            item
        }
    }

    fn pop_timeout(&self, reltime: timespec) -> Option<A> {
        unsafe {
            let mut abstime = uninit();
            if clock_gettime(CLOCK_MONOTONIC, &mut abstime) != 0 {
                abort()
            }
            abstime.tv_sec += reltime.tv_sec;
            abstime.tv_nsec += reltime.tv_nsec;

            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            let mut guard = box.mutex.lock_guard();
            while box.deque.is_empty() {
                if box.not_empty.wait_until_guard(&mut guard, abstime) == Timeout {
                    return None
                }
            }
            let item = box.deque.generic_pop().get();
            box.not_full.signal();
            Some(item)
        }
    }

    fn push(&self, item: A) {
        unsafe {
            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            let mut guard = box.mutex.lock_guard();
            while box.deque.len() == box.maximum {
                box.not_full.wait_guard(&mut guard)
            }
            box.deque.generic_push(item);
            box.not_empty.signal()
        }
    }

    fn push_timeout(&self, item: A, reltime: timespec) -> Option<A> {
        unsafe {
            let mut abstime = uninit();
            if clock_gettime(CLOCK_MONOTONIC, &mut abstime) != 0 {
                abort()
            }
            abstime.tv_sec += reltime.tv_sec;
            abstime.tv_nsec += reltime.tv_nsec;

            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            let mut guard = box.mutex.lock_guard();
            while box.deque.len() == box.maximum {
                if box.not_full.wait_until_guard(&mut guard, abstime) == Timeout {
                    return Some(item)
                }
            }
            box.deque.generic_push(item);
            box.not_empty.signal();
            None
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

impl<T: Send> BoundedQueue<T> {
    /// Return a new `BoundedQueue` instance, holding at most `maximum` elements.
    pub fn new(maximum: uint) -> BoundedQueue<T> {
        BoundedQueue { ptr: BoundedQueuePtr::new(maximum, Deque::new()) }
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty.
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty or the
    /// timeout expires.
    pub fn pop_timeout(&self, reltime: timespec) -> Option<T> {
        self.ptr.pop_timeout(reltime)
    }

    /// Push a value to the back of the queue, blocking until the queue is not full.
    pub fn push(&self, item: T) {
        self.ptr.push(item)
    }

    /// Push a value to the back of the queue, blocking until the queue is not full or the timeout
    /// expires. If the timeout expires, return `Some(item)`.
    pub fn push_timeout(&self, item: T, reltime: timespec) -> Option<T> {
        self.ptr.push_timeout(item, reltime)
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

impl<T: Ord + Send> BoundedPriorityQueue<T> {
    /// Return a new `BoundedPriorityQueue` instance, holding at most `maximum` elements.
    pub fn new(maximum: uint) -> BoundedPriorityQueue<T> {
        BoundedPriorityQueue { ptr: BoundedQueuePtr::new(maximum, PriorityQueue::new()) }
    }

    /// Pop the largest value from the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        self.ptr.pop()
    }

    /// Pop the largest value from the queue, blocking until the queue is not empty or the timeout
    /// expires.
    pub fn pop_timeout(&self, reltime: timespec) -> Option<T> {
        self.ptr.pop_timeout(reltime)
    }

    /// Push a value into the queue, blocking until the queue is not full.
    pub fn push(&self, item: T) {
        self.ptr.push(item)
    }

    /// Push a value into the queue, blocking until the queue is not full or the timeout expires. If
    /// the timeout expires, return `Some(item)`.
    pub fn push_timeout(&self, item: T, reltime: timespec) -> Option<T> {
        self.ptr.push_timeout(item, reltime)
    }
}

impl<T> Clone for BoundedPriorityQueue<T> {
    /// Return a shallow copy of the queue
    fn clone(&self) -> BoundedPriorityQueue<T> {
        BoundedPriorityQueue { ptr: self.ptr.clone() }
    }
}

#[no_freeze]
struct LockedHashMap<K, V> {
    map: HashMap<K, V>,
    mutex: Mutex
}

impl<K: Hash + Eq, V> LockedHashMap<K, V> {
    fn with_capacity_and_keys(k0: u64, k1: u64, capacity: uint) -> LockedHashMap<K, V> {
        LockedHashMap {
            map: HashMap::with_capacity_and_keys(k0, k1, capacity),
            mutex: Mutex::new()
        }
    }

    fn swap(&mut self, k: K, v: V) -> Option<V> {
        unsafe {
            let _guard = self.mutex.lock_guard();
            self.map.swap(k, v)
        }
    }

    fn pop(&mut self, k: &K) -> Option<V> {
        unsafe {
            let _guard = self.mutex.lock_guard();
            self.map.pop(k)
        }
    }
}

impl<K: Hash + Eq, V: Clone> LockedHashMap<K, V> {
    fn find(&mut self, k: &K) -> Option<V> {
        unsafe {
            let _guard = self.mutex.lock_guard();
            self.map.find(k).map(|v| v.clone())
        }
    }
}

/// A concurrent hash table based a single lock per instance
pub struct ConcurrentHashMap<K, V> {
    priv ptr: Arc<LockedHashMap<K, V>>
}

impl<K: Hash + Eq + Send, V: Send> ConcurrentHashMap<K, V> {
    /// Create a new `ConcurrentHashMap` with the specified 128-bit hash key (`k0` and `k1`) and
    /// initial `capacity`.
    pub fn with_capacity_and_keys(k0: u64, k1: u64, capacity: uint) -> ConcurrentHashMap<K, V> {
        let box = LockedHashMap::with_capacity_and_keys(k0, k1, capacity);
        unsafe {
            ConcurrentHashMap { ptr: Arc::new_unchecked(box) }
        }
    }

    /// Insert a key-value pair into the hash table. Return the old value corresponding to the key.
    pub fn swap(&self, k: K, v: V) -> Option<V> {
        unsafe {
            let box: &mut LockedHashMap<K, V> = transmute(self.ptr.borrow());
            box.swap(k, v)
        }
    }

    /// Remove a key-value pair from the map. Return the value corresponding to the key.
    pub fn pop(&self, k: &K) -> Option<V> {
        unsafe {
            let box: &mut LockedHashMap<K, V> = transmute(self.ptr.borrow());
            box.pop(k)
        }
    }
}

impl<K: Hash + Eq, V: Clone> ConcurrentHashMap<K, V> {
    /// Return the value corresponding to the key via `clone`.
    ///
    /// A reference cannot be returned directly, because a lock has to be obtained and released by
    /// the function.
    pub fn find(&self, k: &K) -> Option<V> {
        unsafe {
            let box: &mut LockedHashMap<K, V> = transmute(self.ptr.borrow());
            box.find(k)
        }
    }
}

impl<K, V> Clone for ConcurrentHashMap<K, V> {
    /// Return a shallow copy of the map
    fn clone(&self) -> ConcurrentHashMap<K, V> {
        ConcurrentHashMap { ptr: self.ptr.clone() }
    }
}

#[no_freeze]
struct ShardMapBox<K, V> {
    priv maps: Vec<LockedHashMap<K, V>, Heap>,
    priv k0: u64,
    priv k1: u64
}

impl<K: Hash + Eq, V> ShardMapBox<K, V> {
    fn get_shard(&self, k: &K) -> uint {
        k.hash(self.k0, self.k1) as uint % self.maps.len()
    }
}

/// A concurrent hash table distributing keys across shards, with locking on a per-shard basis
pub struct ShardMap<K, V> {
    priv ptr: Arc<ShardMapBox<K, V>>
}

impl<K: Hash + Eq + Send, V: Send> ShardMap<K, V> {
    /// Create a new `ShardMap` with `shards` internal hash tables, the specified 128-bit hash key
    /// (`k0` and `k1`) and an initial `capacity`.
    pub fn with_capacity_and_keys(shards: uint, k0: u64, k1: u64, capacity: uint) -> ShardMap<K, V> {
        let mut xs = Vec::with_capacity(shards);
        let mut i = 0;
        while i < shards {
            xs.push(LockedHashMap::with_capacity_and_keys(k0, k1, capacity));
            i += 1;
        }
        let box = ShardMapBox { maps: xs, k0: k0, k1: k1 };
        unsafe {
            ShardMap { ptr: Arc::new_unchecked(box) }
        }
    }

    /// Insert a key-value pair into the hash table. Return the old value corresponding to the key.
    pub fn swap(&self, k: K, v: V) -> Option<V> {
        unsafe {
            let box: &mut ShardMapBox<K, V> = transmute(self.ptr.borrow());
            let shard = box.get_shard(&k);
            box.maps.as_mut_slice()[shard].swap(k, v)
        }
    }

    /// Remove a key-value pair from the map. Return the value corresponding to the key.
    pub fn pop(&self, k: &K) -> Option<V> {
        unsafe {
            let box: &mut ShardMapBox<K, V> = transmute(self.ptr.borrow());
            let shard = box.get_shard(k);
            box.maps.as_mut_slice()[shard].pop(k)
        }
    }
}

impl<K: Hash + Eq, V: Clone> ShardMap<K, V> {
    /// Return the value corresponding to the key via `clone`.
    ///
    /// A reference cannot be returned directly, because a lock has to be obtained and released by
    /// the function.
    pub fn find(&self, k: &K) -> Option<V> {
        unsafe {
            let box: &mut ShardMapBox<K, V> = transmute(self.ptr.borrow());
            let shard = box.get_shard(k);
            box.maps.as_mut_slice()[shard].find(k)
        }
    }
}

impl<K, V> Clone for ShardMap<K, V> {
    /// Return a shallow copy of the map
    fn clone(&self) -> ShardMap<K, V> {
        ShardMap { ptr: self.ptr.clone() }
    }
}
