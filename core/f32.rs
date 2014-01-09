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

    extern {
        #[link_name = "llvm.sqrt.f32"]
        pub fn sqrtf32(x: f32) -> f32;
        #[link_name = "llvm.powi.f32"]
        pub fn powif32(a: f32, x: i32) -> f32;
        #[link_name = "llvm.sin.f32"]
        pub fn sinf32(x: f32) -> f32;
        #[link_name = "llvm.cos.f32"]
        pub fn cosf32(x: f32) -> f32;
        #[link_name = "llvm.pow.f32"]
        pub fn powf32(a: f32, x: f32) -> f32;
        #[link_name = "llvm.exp.f32"]
        pub fn expf32(x: f32) -> f32;
        #[link_name = "llvm.exp2.f32"]
        pub fn exp2f32(x: f32) -> f32;
        #[link_name = "llvm.log.f32"]
        pub fn logf32(x: f32) -> f32;
        #[link_name = "llvm.log10.f32"]
        pub fn log10f32(x: f32) -> f32;
        #[link_name = "llvm.log2.f32"]
        pub fn log2f32(x: f32) -> f32;
        #[link_name = "llvm.fma.f32"]
        pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.fabs.f32"]
        pub fn fabsf32(x: f32) -> f32;
        #[link_name = "llvm.copysign.f32"]
        pub fn copysignf32(x: f32, y: f32) -> f32;
        #[link_name = "llvm.floor.f32"]
        pub fn floorf32(x: f32) -> f32;
        #[link_name = "llvm.ceil.f32"]
        pub fn ceilf32(x: f32) -> f32;
        #[link_name = "llvm.trunc.f32"]
        pub fn truncf32(x: f32) -> f32;
        #[link_name = "llvm.rint.f32"]
        pub fn rintf32(x: f32) -> f32;
        #[link_name = "llvm.nearbyint.f32"]
        pub fn nearbyintf32(x: f32) -> f32;
        #[link_name = "llvm.round.f32"]
        pub fn roundf32(x: f32) -> f32;

        pub fn fmaxf(x: f32, y: f32) -> f32;
        pub fn fminf(x: f32, y: f32) -> f32;
        pub fn fdimf(x: f32, y: f32) -> f32;
        pub fn expm1f(x: f32) -> f32;
        pub fn log1pf(x: f32) -> f32;
        pub fn cbrtf(x: f32) -> f32;
        pub fn hypotf(x: f32, y: f32) -> f32;
        pub fn tanf(x: f32) -> f32;
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
        pub fn frexpf(x: f32, exp: *mut c_int) -> f32;
        pub fn ldexpf(x: f32, exp: c_int) -> f32;
        pub fn modff(x: f32, iptr: *mut f32) -> f32;
        pub fn ilogbf(x: f32) -> c_int;
        pub fn logbf(x: f32) -> f32;
        pub fn nextafterf(from: f32, to: f32) -> f32;
        pub fn nexttowardf(from: f32, to: f32) -> f32;
    }
}

#[inline(always)]
pub fn max(x: f32, y: f32) -> f32 {
    unsafe { detail::fmaxf(x, y) }
}

#[inline(always)]
pub fn min(x: f32, y: f32) -> f32 {
    unsafe { detail::fminf(x, y) }
}

#[inline(always)]
pub fn dim(x: f32, y: f32) -> f32 {
    unsafe { detail::fdimf(x, y) }
}

#[inline(always)]
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

#[inline(always)]
pub fn cbrt(x: f32) -> f32 {
    unsafe { detail::cbrtf(x) }
}

#[inline(always)]
pub fn hypot(x: f32, y: f32) -> f32 {
    unsafe { detail::hypotf(x, y) }
}

#[inline(always)]
pub fn powi(x: f32, i: i32) -> f32 {
    unsafe { detail::powif32(x, i) }
}

#[inline(always)]
pub fn pow(x: f32, y: f32) -> f32 {
    unsafe { detail::powf32(x, y) }
}

#[inline(always)]
pub fn exp(x: f32) -> f32 {
    unsafe { detail::expf32(x) }
}

#[inline(always)]
pub fn exp2(x: f32) -> f32 {
    unsafe { detail::exp2f32(x) }
}

#[inline(always)]
pub fn expm1(x: f32) -> f32 {
    unsafe { detail::expm1f(x) }
}

#[inline(always)]
pub fn log(x: f32) -> f32 {
    unsafe { detail::logf32(x) }
}

#[inline(always)]
pub fn log10(x: f32) -> f32 {
    unsafe { detail::log10f32(x) }
}

#[inline(always)]
pub fn log1p(x: f32) -> f32 {
    unsafe { detail::log1pf(x) }
}

#[inline(always)]
pub fn log2(x: f32) -> f32 {
    unsafe { detail::log2f32(x) }
}

#[inline(always)]
pub fn fma(a: f32, b: f32, c: f32) -> f32 {
    unsafe { detail::fmaf32(a, b, c) }
}

#[inline(always)]
pub fn abs(x: f32) -> f32 {
    unsafe { detail::fabsf32(x) }
}

#[inline(always)]
pub fn copysign(x: f32, y: f32) -> f32 {
    unsafe { detail::copysignf32(x, y) }
}

#[inline(always)]
pub fn floor(x: f32) -> f32 {
    unsafe { detail::floorf32(x) }
}

#[inline(always)]
pub fn ceil(x: f32) -> f32 {
    unsafe { detail::ceilf32(x) }
}

#[inline(always)]
pub fn trunc(x: f32) -> f32 {
    unsafe { detail::truncf32(x) }
}

#[inline(always)]
pub fn rint(x: f32) -> f32 {
    unsafe { detail::rintf32(x) }
}

#[inline(always)]
pub fn nearbyint(x: f32) -> f32 {
    unsafe { detail::nearbyintf32(x) }
}

#[inline(always)]
pub fn round(x: f32) -> f32 {
    unsafe { detail::roundf32(x) }
}

#[inline(always)]
pub fn sin(x: f32) -> f32 {
    unsafe { detail::sinf32(x) }
}

#[inline(always)]
pub fn cos(x: f32) -> f32 {
    unsafe { detail::cosf32(x) }
}

#[inline(always)]
pub fn tan(x: f32) -> f32 {
    unsafe { detail::tanf(x) }
}

#[inline(always)]
pub fn asin(x: f32) -> f32 {
    unsafe { detail::asinf(x) }
}

#[inline(always)]
pub fn acos(x: f32) -> f32 {
    unsafe { detail::acosf(x) }
}

#[inline(always)]
pub fn atan(x: f32) -> f32 {
    unsafe { detail::atanf(x) }
}

#[inline(always)]
pub fn atan2(x: f32, y: f32) -> f32 {
    unsafe { detail::atan2f(x, y) }
}

#[inline(always)]
pub fn sinh(x: f32) -> f32 {
    unsafe { detail::sinhf(x) }
}

#[inline(always)]
pub fn cosh(x: f32) -> f32 {
    unsafe { detail::coshf(x) }
}

#[inline(always)]
pub fn tanh(x: f32) -> f32 {
    unsafe { detail::tanhf(x) }
}

#[inline(always)]
pub fn asinh(x: f32) -> f32 {
    unsafe { detail::asinhf(x) }
}

#[inline(always)]
pub fn acosh(x: f32) -> f32 {
    unsafe { detail::acoshf(x) }
}

#[inline(always)]
pub fn atanh(x: f32) -> f32 {
    unsafe { detail::atanhf(x) }
}

#[inline(always)]
pub fn erf(x: f32) -> f32 {
    unsafe { detail::erff(x) }
}

#[inline(always)]
pub fn erfc(x: f32) -> f32 {
    unsafe { detail::erfcf(x) }
}

#[inline(always)]
pub fn tgamma(x: f32) -> f32 {
    unsafe { detail::tgammaf(x) }
}

pub fn frexp(x: f32) -> (f32, c_int) {
    unsafe {
        let mut exp = uninit();
        (detail::frexpf(x, &mut exp), exp)
    }
}

#[inline(always)]
pub fn ldexp(x: f32, exp: c_int) -> f32 {
    unsafe { detail::ldexpf(x, exp) }
}

pub fn modf(x: f32) -> (f32, f32) {
    unsafe {
        let mut i = uninit();
        let frac = detail::modff(x, &mut i);
        (i, frac)
    }
}

#[inline(always)]
pub fn ilogb(x: f32) -> c_int {
    unsafe { detail::ilogbf(x) }
}

#[inline(always)]
pub fn logb(x: f32) -> f32 {
    unsafe { detail::logbf(x) }
}

#[inline(always)]
pub fn nextafter(from: f32, to: f32) -> f32 {
    unsafe { detail::nextafterf(from, to) }
}

#[inline(always)]
pub fn nexttoward(from: f32, to: f32) -> f32 {
    unsafe { detail::nexttowardf(from, to) }
}

#[inline(always)]
pub fn is_nan(x: f64) -> bool {
    x != x
}
