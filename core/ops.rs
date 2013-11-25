// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[lang="drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang="add"]
pub trait Add<RHS, Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

#[lang="sub"]
pub trait Sub<RHS, Result> {
    fn sub(&self, rhs: &RHS) -> Result;
}

#[lang="mul"]
pub trait Mul<RHS, Result> {
    fn mul(&self, rhs: &RHS) -> Result;
}

#[lang="div"]
pub trait Div<RHS, Result> {
    fn div(&self, rhs: &RHS) -> Result;
}

#[lang="rem"]
pub trait Rem<RHS, Result> {
    fn rem(&self, rhs: &RHS) -> Result;
}

#[lang="neg"]
pub trait Neg<Result> {
    fn neg(&self) -> Result;
}

#[lang="not"]
pub trait Not<Result> {
    fn not(&self) -> Result;
}

#[lang="bitand"]
pub trait BitAnd<RHS, Result> {
    fn bitand(&self, rhs: &RHS) -> Result;
}

#[lang="bitor"]
pub trait BitOr<RHS, Result> {
    fn bitor(&self, rhs: &RHS) -> Result;
}

#[lang="bitxor"]
pub trait BitXor<RHS, Result> {
    fn bitxor(&self, rhs: &RHS) -> Result;
}

#[lang="shl"]
pub trait Shl<RHS, Result> {
    fn shl(&self, rhs: &RHS) -> Result;
}

#[lang="shr"]
pub trait Shr<RHS, Result> {
    fn shr(&self, rhs: &RHS) -> Result;
}

#[lang="index"]
pub trait Index<Index, Result> {
    fn index(&self, index: &Index) -> Result;
}

#[lang="eq"]
pub trait Eq {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool { !self.ne(other) }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool { !self.eq(other) }
}

impl Eq for int {
    #[inline(always)]
    fn eq(&self, other: &int) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &int) -> bool { *self != *other }
}

impl Eq for i8 {
    #[inline(always)]
    fn eq(&self, other: &i8) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &i8) -> bool { *self != *other }
}

impl Eq for i16 {
    #[inline(always)]
    fn eq(&self, other: &i16) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &i16) -> bool { *self != *other }
}

impl Eq for i32 {
    #[inline(always)]
    fn eq(&self, other: &i32) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &i32) -> bool { *self != *other }
}

impl Eq for i64 {
    #[inline(always)]
    fn eq(&self, other: &i64) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &i64) -> bool { *self != *other }
}

impl Eq for uint {
    #[inline(always)]
    fn eq(&self, other: &uint) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &uint) -> bool { *self != *other }
}

impl Eq for u8 {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &u8) -> bool { *self != *other }
}

impl Eq for u16 {
    #[inline(always)]
    fn eq(&self, other: &u16) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &u16) -> bool { *self != *other }
}

impl Eq for u32 {
    #[inline(always)]
    fn eq(&self, other: &u32) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &u32) -> bool { *self != *other }
}

impl Eq for u64 {
    #[inline(always)]
    fn eq(&self, other: &u64) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &u64) -> bool { *self != *other }
}

impl Eq for () {
    #[inline(always)]
    fn eq(&self, _: &()) -> bool { true }

    #[inline(always)]
    fn ne(&self, _: &()) -> bool { true }
}

impl Eq for bool {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &bool) -> bool { *self != *other }
}

impl Eq for char {
    #[inline(always)]
    fn eq(&self, other: &char) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &char) -> bool { *self != *other }
}

#[lang="ord"]
pub trait Ord {
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool { other.gt(self) }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool { !other.lt(self) }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool { other.lt(self) }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool { !self.lt(other) }
}

impl Ord for int {
    #[inline(always)]
    fn lt(&self, other: &int) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &int) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &int) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &int) -> bool { *self >= *other }
}

impl Ord for i8 {
    #[inline(always)]
    fn lt(&self, other: &i8) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &i8) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &i8) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &i8) -> bool { *self >= *other }
}

impl Ord for i16 {
    #[inline(always)]
    fn lt(&self, other: &i16) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &i16) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &i16) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &i16) -> bool { *self >= *other }
}

impl Ord for i32 {
    #[inline(always)]
    fn lt(&self, other: &i32) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &i32) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &i32) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &i32) -> bool { *self >= *other }
}

impl Ord for i64 {
    #[inline(always)]
    fn lt(&self, other: &i64) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &i64) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &i64) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &i64) -> bool { *self >= *other }
}

impl Ord for uint {
    #[inline(always)]
    fn lt(&self, other: &uint) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &uint) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &uint) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &uint) -> bool { *self >= *other }
}

impl Ord for u8 {
    #[inline(always)]
    fn lt(&self, other: &u8) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &u8) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &u8) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &u8) -> bool { *self >= *other }
}

impl Ord for u16 {
    #[inline(always)]
    fn lt(&self, other: &u16) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &u16) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &u16) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &u16) -> bool { *self >= *other }
}

impl Ord for u32 {
    #[inline(always)]
    fn lt(&self, other: &u32) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &u32) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &u32) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &u32) -> bool { *self >= *other }
}

impl Ord for u64 {
    #[inline(always)]
    fn lt(&self, other: &u64) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &u64) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &u64) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &u64) -> bool { *self >= *other }
}

impl Ord for () {
    #[inline(always)]
    fn lt(&self, _: &()) -> bool { false }

    #[inline(always)]
    fn le(&self, _: &()) -> bool { true }

    #[inline(always)]
    fn gt(&self, _: &()) -> bool { false }

    #[inline(always)]
    fn ge(&self, _: &()) -> bool { true }
}

impl Ord for bool {
    #[inline(always)]
    fn lt(&self, other: &bool) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &bool) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &bool) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &bool) -> bool { *self >= *other }
}

impl Ord for char {
    #[inline(always)]
    fn lt(&self, other: &char) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &char) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &char) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &char) -> bool { *self >= *other }
}

impl<T: Ord> Ord for ~T {
    #[inline(always)]
    fn lt(&self, other: &~T) -> bool { **self < **other }

    #[inline(always)]
    fn le(&self, other: &~T) -> bool { **self <= **other }

    #[inline(always)]
    fn gt(&self, other: &~T) -> bool { **self > **other }

    #[inline(always)]
    fn ge(&self, other: &~T) -> bool { **self >= **other }
}
