// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::fail::abort;

pub enum Option<T> {
    Some(T),
    None
}

impl<T> Option<T> {
    /// Convert from `Option<T>` to `Option<&T>`
    pub fn as_ref<'r>(&'r self) -> Option<&'r T> {
        match *self { Some(ref x) => Some(x), None => None }
    }

    /// Convert from `Option<T>` to `Option<&mut T>`
    pub fn as_mut<'r>(&'r mut self) -> Option<&'r mut T> {
        match *self { Some(ref mut x) => Some(x), None => None }
    }

    /// Return the value in an `Option` or call `abort` if it is `None`.
    pub fn get(self) -> T {
        match self { Some(x) => x, None => abort() }
    }

    /// Maps an `Option<T>` to `Option<U>` by applying a function to a contained value.
    pub fn map<U>(self, f: &fn(T) -> U) -> Option<U> {
        match self { Some(x) => Some(f(x)), None => None }
    }

    /// Applies a function to the contained value or returns a default.
    pub fn map_or<U>(self, def: U, f: &fn(T) -> U) -> U {
        match self { None => def, Some(t) => f(t) }
    }
}
