
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
// we need this to be able to inclde FieldOffsets in C structs
#![allow(improper_ctypes)]
// we have a bunch of unused code during testing at the moment, somehow
#![cfg_attr(test, allow(unused))]
#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(concat_idents)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]
#![feature(untagged_unions)]

pub mod remacs_sys;
#[macro_use]
pub mod lisp;
pub mod multibyte;
pub mod numbers;