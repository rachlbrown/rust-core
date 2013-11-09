// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::fail::assert;
use super::mem::size_of;
use super::heap::malloc_raw;

#[lang="opaque"]
enum Opaque {}

type Disr = u64;

#[lang="ty_desc"]
struct TyDesc {
    size: uint,
    align: uint,
    take_glue: extern "Rust" fn(*i8),
    drop_glue: extern "Rust" fn(*i8),
    free_glue: extern "Rust" fn(*i8),
    visit_glue: extern "Rust" fn(*i8),
    borrow_offset: uint,
    name: &'static str
}

#[lang="ty_visitor"]
trait TyVisitor {
    fn visit_bot(&mut self) -> bool;
    fn visit_nil(&mut self) -> bool;
    fn visit_bool(&mut self) -> bool;

    fn visit_int(&mut self) -> bool;
    fn visit_i8(&mut self) -> bool;
    fn visit_i16(&mut self) -> bool;
    fn visit_i32(&mut self) -> bool;
    fn visit_i64(&mut self) -> bool;

    fn visit_uint(&mut self) -> bool;
    fn visit_u8(&mut self) -> bool;
    fn visit_u16(&mut self) -> bool;
    fn visit_u32(&mut self) -> bool;
    fn visit_u64(&mut self) -> bool;

    fn visit_f32(&mut self) -> bool;
    fn visit_f64(&mut self) -> bool;

    fn visit_char(&mut self) -> bool;

    fn visit_estr_box(&mut self) -> bool;
    fn visit_estr_uniq(&mut self) -> bool;
    fn visit_estr_slice(&mut self) -> bool;
    fn visit_estr_fixed(&mut self, n: uint, sz: uint, align: uint) -> bool;

    fn visit_box(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq_managed(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_ptr(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_rptr(&mut self, mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_vec(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_unboxed_vec(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_box(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq_managed(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_slice(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_fixed(&mut self, n: uint, sz: uint, align: uint,
                        mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_enter_rec(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_rec_field(&mut self, i: uint, name: &str,
                       mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_rec(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_class(&mut self, name: &str, named_fields: bool, n_fields: uint,
                         sz: uint, align: uint) -> bool;
    fn visit_class_field(&mut self, i: uint, name: &str, named: bool,
                         mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_class(&mut self, name: &str, named_fields: bool, n_fields: uint,
                         sz: uint, align: uint) -> bool;

    fn visit_enter_tup(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_tup_field(&mut self, i: uint, inner: *TyDesc) -> bool;
    fn visit_leave_tup(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_enum(&mut self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> Disr,
                        sz: uint, align: uint) -> bool;
    fn visit_enter_enum_variant(&mut self, variant: uint,
                                disr_val: Disr,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_enum_variant_field(&mut self, i: uint, offset: uint, inner: *TyDesc) -> bool;
    fn visit_leave_enum_variant(&mut self, variant: uint,
                                disr_val: Disr,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_leave_enum(&mut self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> Disr,
                        sz: uint, align: uint) -> bool;

    fn visit_enter_fn(&mut self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;
    fn visit_fn_input(&mut self, i: uint, mode: uint, inner: *TyDesc) -> bool;
    fn visit_fn_output(&mut self, retstyle: uint, variadic: bool, inner: *TyDesc) -> bool;
    fn visit_leave_fn(&mut self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;

    fn visit_trait(&mut self, name: &str) -> bool;
    fn visit_param(&mut self, i: uint) -> bool;
    fn visit_self(&mut self) -> bool;
    fn visit_type(&mut self) -> bool;
    fn visit_opaque_box(&mut self) -> bool;
    fn visit_closure_ptr(&mut self, ck: uint) -> bool;
}

struct Box<T> {
    ref_count: uint,
    type_desc: *TyDesc,
    prev: *mut Box<T>,
    next: *mut Box<T>,
    data: T
}

fn get_box_size(body_size: uint, body_align: uint) -> uint {
    let header_size = size_of::<Box<()>>();
    let total_size = align_to(header_size, body_align) + body_size;
    total_size
}

fn align_to(size: uint, align: uint) -> uint {
    assert(align != 0);
    (size + align - 1) & !(align - 1)
}

#[lang="closure_exchange_malloc"]
pub unsafe fn closure_exchange_malloc(td: *u8, size: uint) -> *u8 {
    let td = td as *TyDesc;

    assert(td as uint != 0);

    let total_size = get_box_size(size, (*td).align);
    let p = malloc_raw(total_size as uint);

    let box = p as *mut Box<()>;
    (*box).type_desc = td;

    box as *u8
}
