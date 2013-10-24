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
        pub fn copysignf32(x: f32, y: f32) -> f32;
        pub fn floorf32(x: f32) -> f32;
        pub fn ceilf32(x: f32) -> f32;
        pub fn truncf32(x: f32) -> f32;
        pub fn rintf32(x: f32) -> f32;
        pub fn nearbyintf32(x: f32) -> f32;
        pub fn roundf32(x: f32) -> f32;
    }

    extern {
        pub fn expm1f(x: f32) -> f32;
        pub fn cbrtf(x: f32) -> f32;
        pub fn asinf(x: f32) -> f32;
        pub fn acosf(x: f32) -> f32;
        pub fn atanf(x: f32) -> f32;
        pub fn atan2f(x: f32, y: f32) -> f32;
        pub fn sinhf(x: f32) -> f32;
        pub fn coshf(x: f32) -> f32;
        pub fn tanhf(x: f32) -> f32;
        pub fn asinhf(x: f32) -> f32;
        pub fn acoshf(x: f32) -> f32;
        pub fn atanhf(x: f32) -> f32;
        pub fn erff(x: f32) -> f32;
        pub fn erfcf(x: f32) -> f32;
        pub fn tgammaf(x: f32) -> f32;
    }
}

pub unsafe fn sqrt_unchecked(x: f32) -> f32 {
    detail::sqrtf32(x)
}

pub fn sqrt(x: f32) -> f32 {
    if x < -0.0 {
        0.0 / 0.0 // evaluate to NaN and raise `FE_INVALID`
    } else {
        unsafe { detail::sqrtf32(x) }
    }
}

pub fn cbrt(x: f32) -> f32 {
    unsafe { detail::cbrtf(x) }
}

pub fn powi(x: f32, i: i32) -> f32 {
    unsafe { detail::powif32(x, i) }
}

pub fn sin(x: f32) -> f32 {
    unsafe { detail::sinf32(x) }
}

pub fn cos(x: f32) -> f32 {
    unsafe { detail::cosf32(x) }
}

pub fn pow(x: f32, y: f32) -> f32 {
    unsafe { detail::powf32(x, y) }
}

pub fn exp(x: f32) -> f32 {
    unsafe { detail::expf32(x) }
}

pub fn exp2(x: f32) -> f32 {
    unsafe { detail::exp2f32(x) }
}

pub fn expm1(x: f32) -> f32 {
    unsafe { detail::expm1f(x) }
}

pub fn log(x: f32) -> f32 {
    unsafe { detail::logf32(x) }
}

pub fn log10(x: f32) -> f32 {
    unsafe { detail::log10f32(x) }
}

pub fn log2(x: f32) -> f32 {
    unsafe { detail::log2f32(x) }
}

pub fn fma(a: f32, b: f32, c: f32) -> f32 {
    unsafe { detail::fmaf32(a, b, c) }
}

pub fn abs(x: f32) -> f32 {
    unsafe { detail::fabsf32(x) }
}

pub fn copysign(x: f32, y: f32) -> f32 {
    unsafe { detail::copysignf32(x, y) }
}

pub fn floor(x: f32) -> f32 {
    unsafe { detail::floorf32(x) }
}

pub fn ceil(x: f32) -> f32 {
    unsafe { detail::ceilf32(x) }
}

pub fn trunc(x: f32) -> f32 {
    unsafe { detail::truncf32(x) }
}

pub fn rint(x: f32) -> f32 {
    unsafe { detail::rintf32(x) }
}

pub fn nearbyint(x: f32) -> f32 {
    unsafe { detail::nearbyintf32(x) }
}

pub fn round(x: f32) -> f32 {
    unsafe { detail::roundf32(x) }
}

pub fn asin(x: f32) -> f32 {
    unsafe { detail::asinf(x) }
}

pub fn acos(x: f32) -> f32 {
    unsafe { detail::acosf(x) }
}

pub fn atan(x: f32) -> f32 {
    unsafe { detail::atanf(x) }
}

pub fn atan2(x: f32, y: f32) -> f32 {
    unsafe { detail::atan2f(x, y) }
}

pub fn sinh(x: f32) -> f32 {
    unsafe { detail::sinhf(x) }
}

pub fn cosh(x: f32) -> f32 {
    unsafe { detail::coshf(x) }
}

pub fn tanh(x: f32) -> f32 {
    unsafe { detail::tanhf(x) }
}

pub fn asinh(x: f32) -> f32 {
    unsafe { detail::asinhf(x) }
}

pub fn acosh(x: f32) -> f32 {
    unsafe { detail::acoshf(x) }
}

pub fn atanh(x: f32) -> f32 {
    unsafe { detail::atanhf(x) }
}

pub fn erf(x: f32) -> f32 {
    unsafe { detail::erff(x) }
}

pub fn erfc(x: f32) -> f32 {
    unsafe { detail::erfcf(x) }
}

pub fn tgamma(x: f32) -> f32 {
    unsafe { detail::tgammaf(x) }
}
