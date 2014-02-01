// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use c_types::c_int;
use mem::uninit;

mod detail {
    use c_types::c_int;

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

    extern {
        pub fn fmax(x: f64, y: f64) -> f64;
        pub fn fmin(x: f64, y: f64) -> f64;
        pub fn fdim(x: f64, y: f64) -> f64;
        pub fn expm1(x: f64) -> f64;
        pub fn log1p(x: f64) -> f64;
        pub fn cbrt(x: f64) -> f64;
        pub fn hypot(x: f64, y: f64) -> f64;
        pub fn tan(x: f64) -> f64;
        pub fn asin(x: f64) -> f64;
        pub fn acos(x: f64) -> f64;
        pub fn atan(x: f64) -> f64;
        pub fn atan2(x: f64, y: f64) -> f64;
        pub fn sinh(x: f64) -> f64;
        pub fn cosh(x: f64) -> f64;
        pub fn tanh(x: f64) -> f64;
        pub fn asinh(x: f64) -> f64;
        pub fn acosh(x: f64) -> f64;
        pub fn atanh(x: f64) -> f64;
        pub fn erf(x: f64) -> f64;
        pub fn erfc(x: f64) -> f64;
        pub fn tgamma(x: f64) -> f64;
        pub fn frexp(x: f64, exp: *mut c_int) -> f64;
        pub fn ldexp(x: f64, exp: c_int) -> f64;
        pub fn modf(x: f64, iptr: *mut f64) -> f64;
        pub fn ilogb(x: f64) -> c_int;
        pub fn logb(x: f64) -> f64;
        pub fn nextafter(from: f64, to: f64) -> f64;
        pub fn nexttoward(from: f64, to: f64) -> f64;
    }
}

#[inline(always)]
pub fn max(x: f64, y: f64) -> f64 {
    unsafe { detail::fmax(x, y) }
}

#[inline(always)]
pub fn min(x: f64, y: f64) -> f64 {
    unsafe { detail::fmin(x, y) }
}

#[inline(always)]
pub fn dim(x: f64, y: f64) -> f64 {
    unsafe { detail::fdim(x, y) }
}

#[inline(always)]
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

#[inline(always)]
pub fn cbrt(x: f64) -> f64 {
    unsafe { detail::cbrt(x) }
}

#[inline(always)]
pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { detail::hypot(x, y) }
}

#[inline(always)]
pub fn powi(x: f64, i: i32) -> f64 {
    unsafe { detail::powif64(x, i) }
}

#[inline(always)]
pub fn pow(x: f64, y: f64) -> f64 {
    unsafe { detail::powf64(x, y) }
}

#[inline(always)]
pub fn exp(x: f64) -> f64 {
    unsafe { detail::expf64(x) }
}

#[inline(always)]
pub fn exp2(x: f64) -> f64 {
    unsafe { detail::exp2f64(x) }
}

#[inline(always)]
pub fn expm1(x: f64) -> f64 {
    unsafe { detail::expm1(x) }
}

#[inline(always)]
pub fn log(x: f64) -> f64 {
    unsafe { detail::logf64(x) }
}

#[inline(always)]
pub fn log10(x: f64) -> f64 {
    unsafe { detail::log10f64(x) }
}

#[inline(always)]
pub fn log1p(x: f64) -> f64 {
    unsafe { detail::log1p(x) }
}

#[inline(always)]
pub fn log2(x: f64) -> f64 {
    unsafe { detail::log2f64(x) }
}

#[inline(always)]
pub fn fma(a: f64, b: f64, c: f64) -> f64 {
    unsafe { detail::fmaf64(a, b, c) }
}

#[inline(always)]
pub fn abs(x: f64) -> f64 {
    unsafe { detail::fabsf64(x) }
}

#[inline(always)]
pub fn copysign(x: f64, y: f64) -> f64 {
    unsafe { detail::copysignf64(x, y) }
}

#[inline(always)]
pub fn floor(x: f64) -> f64 {
    unsafe { detail::floorf64(x) }
}

#[inline(always)]
pub fn ceil(x: f64) -> f64 {
    unsafe { detail::ceilf64(x) }
}

#[inline(always)]
pub fn trunc(x: f64) -> f64 {
    unsafe { detail::truncf64(x) }
}

#[inline(always)]
pub fn rint(x: f64) -> f64 {
    unsafe { detail::rintf64(x) }
}

#[inline(always)]
pub fn nearbyint(x: f64) -> f64 {
    unsafe { detail::nearbyintf64(x) }
}

#[inline(always)]
pub fn round(x: f64) -> f64 {
    unsafe { detail::roundf64(x) }
}

#[inline(always)]
pub fn sin(x: f64) -> f64 {
    unsafe { detail::sinf64(x) }
}

#[inline(always)]
pub fn cos(x: f64) -> f64 {
    unsafe { detail::cosf64(x) }
}

#[inline(always)]
pub fn tan(x: f64) -> f64 {
    unsafe { detail::tan(x) }
}

#[inline(always)]
pub fn asin(x: f64) -> f64 {
    unsafe { detail::asin(x) }
}

#[inline(always)]
pub fn acos(x: f64) -> f64 {
    unsafe { detail::acos(x) }
}

#[inline(always)]
pub fn atan(x: f64) -> f64 {
    unsafe { detail::atan(x) }
}

#[inline(always)]
pub fn atan2(x: f64, y: f64) -> f64 {
    unsafe { detail::atan2(x, y) }
}

#[inline(always)]
pub fn sinh(x: f64) -> f64 {
    unsafe { detail::sinh(x) }
}

#[inline(always)]
pub fn cosh(x: f64) -> f64 {
    unsafe { detail::cosh(x) }
}

#[inline(always)]
pub fn tanh(x: f64) -> f64 {
    unsafe { detail::tanh(x) }
}

#[inline(always)]
pub fn asinh(x: f64) -> f64 {
    unsafe { detail::asinh(x) }
}

#[inline(always)]
pub fn acosh(x: f64) -> f64 {
    unsafe { detail::acosh(x) }
}

#[inline(always)]
pub fn atanh(x: f64) -> f64 {
    unsafe { detail::atanh(x) }
}

#[inline(always)]
pub fn erf(x: f64) -> f64 {
    unsafe { detail::erf(x) }
}

#[inline(always)]
pub fn erfc(x: f64) -> f64 {
    unsafe { detail::erfc(x) }
}

#[inline(always)]
pub fn tgamma(x: f64) -> f64 {
    unsafe { detail::tgamma(x) }
}

pub fn frexp(x: f64) -> (f64, c_int) {
    unsafe {
        let mut exp = uninit();
        (detail::frexp(x, &mut exp), exp)
    }
}

#[inline(always)]
pub fn ldexp(x: f64, exp: c_int) -> f64 {
    unsafe { detail::ldexp(x, exp) }
}

pub fn modf(x: f64) -> (f64, f64) {
    unsafe {
        let mut i = uninit();
        let frac = detail::modf(x, &mut i);
        (i, frac)
    }
}

#[inline(always)]
pub fn ilogb(x: f64) -> c_int {
    unsafe { detail::ilogb(x) }
}

#[inline(always)]
pub fn logb(x: f64) -> f64 {
    unsafe { detail::logb(x) }
}

#[inline(always)]
pub fn nextafter(from: f64, to: f64) -> f64 {
    unsafe { detail::nextafter(from, to) }
}

#[inline(always)]
pub fn nexttoward(from: f64, to: f64) -> f64 {
    unsafe { detail::nexttoward(from, to) }
}

#[inline(always)]
pub fn is_nan(x: f64) -> bool {
    x != x
}
