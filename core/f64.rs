// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern "rust-intrinsic" {
    pub fn sqrtf64(x: f64) -> f64;
    pub fn powif64(a: f64, x: i32) -> f64;
    pub fn sinf64(x: f64) -> f64;
    pub fn cosf64(x: f64) -> f64;
    pub fn powf64(a: f64, x: f64) -> f64;
    pub fn expf64(x: f64) -> f64;
    pub fn exp2f64(x: f64) -> f64;
    pub fn logf64(x: f64) -> f64;
    pub fn log10f64(x: f64) -> f64;
    pub fn log2f64(x: f64) -> f64;
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;
    pub fn fabsf64(x: f64) -> f64;
    pub fn floorf64(x: f64) -> f64;
    pub fn ceilf64(x: f64) -> f64;
    pub fn truncf64(x: f64) -> f64;
}
