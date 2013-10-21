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
    pub fn sqrtf32(x: f32) -> f32;
    pub fn powif32(a: f32, x: i32) -> f32;
    pub fn sinf32(x: f32) -> f32;
    pub fn cosf32(x: f32) -> f32;
    pub fn powf32(a: f32, x: f32) -> f32;
    pub fn expf32(x: f32) -> f32;
    pub fn exp2f32(x: f32) -> f32;
    pub fn logf32(x: f32) -> f32;
    pub fn log10f32(x: f32) -> f32;
    pub fn log2f32(x: f32) -> f32;
    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    pub fn fabsf32(x: f32) -> f32;
    pub fn floorf32(x: f32) -> f32;
    pub fn ceilf32(x: f32) -> f32;
    pub fn truncf32(x: f32) -> f32;
}
