// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod detail {
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
        pub fn copysignf64(x: f64, y: f64) -> f64;
        pub fn floorf64(x: f64) -> f64;
        pub fn ceilf64(x: f64) -> f64;
        pub fn truncf64(x: f64) -> f64;
        pub fn rintf64(x: f64) -> f64;
        pub fn nearbyintf64(x: f64) -> f64;
        pub fn roundf64(x: f64) -> f64;
    }
}

pub unsafe fn sqrt_unchecked(x: f64) -> f64 {
    detail::sqrtf64(x)
}

pub fn sqrt(x: f64) -> f64 {
    if x < -0.0 {
        0.0 / 0.0 // evaluate to NaN and raise `FE_INVALID`
    } else {
        unsafe { detail::sqrtf64(x) }
    }
}

pub fn powi(x: f64, i: i32) -> f64 {
    unsafe { detail::powif64(x, i) }
}

pub fn sin(x: f64) -> f64 {
    unsafe { detail::sinf64(x) }
}

pub fn cos(x: f64) -> f64 {
    unsafe { detail::cosf64(x) }
}

pub fn pow(x: f64, y: f64) -> f64 {
    unsafe { detail::powf64(x, y) }
}

pub fn exp(x: f64) -> f64 {
    unsafe { detail::expf64(x) }
}

pub fn exp2(x: f64) -> f64 {
    unsafe { detail::exp2f64(x) }
}

pub fn log(x: f64) -> f64 {
    unsafe { detail::logf64(x) }
}

pub fn log10(x: f64) -> f64 {
    unsafe { detail::log10f64(x) }
}

pub fn log2(x: f64) -> f64 {
    unsafe { detail::log2f64(x) }
}

pub fn fma(a: f64, b: f64, c: f64) -> f64 {
    unsafe { detail::fmaf64(a, b, c) }
}

pub fn abs(x: f64) -> f64 {
    unsafe { detail::fabsf64(x) }
}

pub fn copysign(x: f64, y: f64) -> f64 {
    unsafe { detail::copysignf64(x, y) }
}

pub fn floor(x: f64) -> f64 {
    unsafe { detail::floorf64(x) }
}

pub fn ceil(x: f64) -> f64 {
    unsafe { detail::ceilf64(x) }
}

pub fn trunc(x: f64) -> f64 {
    unsafe { detail::truncf64(x) }
}

pub fn rint(x: f64) -> f64 {
    unsafe { detail::rintf64(x) }
}

pub fn nearbyint(x: f64) -> f64 {
    unsafe { detail::nearbyintf64(x) }
}

pub fn round(x: f64) -> f64 {
    unsafe { detail::roundf64(x) }
}
