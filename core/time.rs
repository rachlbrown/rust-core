// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use fail::{EINTR, abort, assert};
use mem::uninit;
use c_types::{c_int, clockid_t, timespec};

static CLOCK_REALTIME: clockid_t = 0;
static CLOCK_MONOTONIC: clockid_t = 1;
static TIMER_ABSTIME: c_int = 1;

extern {
    fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> c_int;
    fn clock_nanosleep(clock_id: clockid_t, flags: c_int, rqtp: *timespec,
                       rmtp: *mut timespec) -> c_int;
}

fn get_time(clock: clockid_t) -> timespec {
    unsafe {
        let mut time = uninit();
        if clock_gettime(clock, &mut time) != 0 {
            abort()
        }
        time
    }
}

pub fn real() -> timespec {
    get_time(CLOCK_REALTIME)
}

pub fn monotonic() -> timespec {
    get_time(CLOCK_MONOTONIC)
}

fn sleep_absolute(clock_id: clockid_t, abstime: timespec) {
    loop {
        let ret = unsafe {
            clock_nanosleep(clock_id, TIMER_ABSTIME, &abstime, 0 as *mut timespec)
        };
        if ret != EINTR {
            assert(ret == 0);
            break
        }
    }
}

pub fn sleep_until(abstime: timespec) {
    sleep_absolute(CLOCK_REALTIME, abstime)
}

pub fn monotonic_sleep_until(abstime: timespec) {
    sleep_absolute(CLOCK_MONOTONIC, abstime)
}

pub fn sleep(mut reltime: timespec) {
    loop {
        unsafe {
            let mut remain = uninit();
            let ret = clock_nanosleep(CLOCK_MONOTONIC, 0, &reltime, &mut remain);
            if ret != EINTR {
                assert(ret == 0);
                break
            }
            reltime = remain;
        }
    }
}
