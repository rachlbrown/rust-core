// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Equivalence relation
///
/// * `a != b` returns the same value as `!(a == b)`
/// * `a == a` is true
/// * `a == b` implies `b == a`
/// * `a == b && b == c` implies `a == c`
#[lang="eq"]
pub trait Eq {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool { !self.ne(other) }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool { !self.eq(other) }
}

macro_rules! eq_impl(
    ($t:ty) => {
        impl Eq for $t {
            #[inline(always)]
            fn eq(&self, other: &$t) -> bool { *self == *other }

            #[inline(always)]
            fn ne(&self, other: &$t) -> bool { *self != *other }
        }
    }
)

eq_impl!(int)
eq_impl!(i8)
eq_impl!(i16)
eq_impl!(i32)
eq_impl!(i64)

eq_impl!(uint)
eq_impl!(u8)
eq_impl!(u16)
eq_impl!(u32)
eq_impl!(u64)

eq_impl!(())
eq_impl!(bool)
eq_impl!(char)

impl<'a, T: Eq> Eq for &'a T {
    #[inline(always)]
    fn eq(&self, other: & &T) -> bool { **self == **other }

    #[inline(always)]
    fn ne(&self, other: & &T) -> bool { **self != **other }
}

impl<T: Eq> Eq for ~T {
    #[inline(always)]
    fn eq(&self, other: &~T) -> bool { **self == **other }

    #[inline(always)]
    fn ne(&self, other: &~T) -> bool { **self != **other }
}

pub enum Ordering {
    Less,
    Equal,
    Greater
}

/// Total ordering
#[lang="ord"]
pub trait Ord: Eq {
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool { other.gt(self) }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool { !other.lt(self) }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool { other.lt(self) }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool { !self.lt(other) }

    fn cmp(&self, other: &Self) -> Ordering {
        if *self < *other { Less }
        else if *self > *other { Greater }
        else { Equal }
    }
}

macro_rules! ord_impl(
    ($t:ty) => {
        impl Ord for $t {
            #[inline(always)]
            fn lt(&self, other: &$t) -> bool { *self < *other }

            #[inline(always)]
            fn le(&self, other: &$t) -> bool { *self <= *other }

            #[inline(always)]
            fn gt(&self, other: &$t) -> bool { *self > *other }

            #[inline(always)]
            fn ge(&self, other: &$t) -> bool { *self >= *other }
        }
    }
)

ord_impl!(int)
ord_impl!(i8)
ord_impl!(i16)
ord_impl!(i32)
ord_impl!(i64)

ord_impl!(uint)
ord_impl!(u8)
ord_impl!(u16)
ord_impl!(u32)
ord_impl!(u64)

ord_impl!(())
ord_impl!(bool)
ord_impl!(char)

impl<'a, T: Ord> Ord for &'a T {
    #[inline(always)]
    fn lt(&self, other: & &T) -> bool { **self < **other }

    #[inline(always)]
    fn le(&self, other: & &T) -> bool { **self <= **other }

    #[inline(always)]
    fn gt(&self, other: & &T) -> bool { **self > **other }

    #[inline(always)]
    fn ge(&self, other: & &T) -> bool { **self >= **other }
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

#[inline(always)]
pub fn max<T: Ord>(x: T, y: T) -> T {
    if x < y { y } else { x }
}

#[inline(always)]
pub fn min<T: Ord>(x: T, y: T) -> T {
    if x < y { x } else { y }
}
