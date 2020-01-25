#![allow(non_snake_case)]
use std::mem;
use std::os::raw::{c_char, c_int};

use ffi::mpz_t;
pub use ffi::{element_t, field_s, pairing_t};

#[allow(dead_code, non_camel_case_types, unused_imports, non_snake_case)]
mod ffi;

#[link(name = "pbc")]
extern "C" {
    // 0 on success
    pub fn pairing_init_set_buf(pairing: &mut pairing_t, input: *const c_char, len: usize)
        -> c_int;
}
#[link(name = "gmp")]
extern "C" {
    fn __gmpz_init(_: &mut mpz_t);
    fn __gmpz_clear(_: &mut mpz_t);
}

pub unsafe fn element_init(e: *mut element_t, f: *const field_s) {
    (*e)[0].field = f;
    (*f).init.as_ref().unwrap()(&mut (*e)[0]);
}

pub unsafe fn element_init_G2(e: *mut element_t, pairing: &pairing_t) {
    element_init(e, pairing[0].G2);
}

pub unsafe fn element_init_G1(e: *mut element_t, pairing: &pairing_t) {
    element_init(e, pairing[0].G1);
}

pub unsafe fn element_init_GT(e: *mut element_t, pairing: &pairing_t) {
    element_init(e, &pairing[0].GT[0]);
}

pub unsafe fn element_init_Zr(e: *mut element_t, pairing: &pairing_t) {
    element_init(e, &pairing[0].Zr[0]);
}

/// Converts 'e' to a GMP integer 'z'
/// if such an operation makes sense
pub unsafe fn element_to_mpz(z: &mut mpz_t, e: &element_t) {
    (*e[0].field).to_mpz.as_ref().unwrap()(&mut z[0], &e[0]);
}

pub unsafe fn element_pow_mpz(x: &mut element_t, a: &element_t, n: &mpz_t) {
    // PBC_ASSERT_MATCH2(x, a)
    (*x[0].field).pow_mpz.as_ref().unwrap()(&mut x[0], &a[0], &n[0]);
}

/// Set 'x' = 'a'^'n'^, where 'n' is an element of a ring *Z*~N~
/// for some 'N' (typically the order of the algebraic structure 'x' lies in).
pub unsafe fn element_pow_zn(x: &mut element_t, a: &element_t, n: &element_t) {
    let mut z: mpz_t = mem::zeroed();
    // PBC_ASSERT_MATCH2(x, a);
    __gmpz_init(&mut z);
    element_to_mpz(&mut z, n);
    element_pow_mpz(x, a, &z);
    __gmpz_clear(&mut z);
}

pub unsafe fn element_from_hash(e: &mut element_t, data: &[u8]) {
    (*e[0].field).from_hash.as_ref().unwrap()(&mut e[0], data.as_ptr() as _, data.len() as _);
}

pub unsafe fn element_cmp(a: &element_t, b: &element_t) -> i32 {
    // PBC_ASSERT_MATCH2(a, b)
    (*a[0].field).cmp.as_ref().unwrap()(&a[0], &b[0])
}

pub unsafe fn element_length_in_bytes(e: &element_t) -> i32 {
    if (*e[0].field).fixed_length_in_bytes < 0 {
        return (*e[0].field).length_in_bytes.as_ref().unwrap()(&e[0]);
    }
    return (*e[0].field).fixed_length_in_bytes;
}

pub unsafe fn element_to_bytes(e: &element_t) -> Vec<u8> {
    let len = element_length_in_bytes(e);
    let mut buf = vec![0u8; len as usize];
    // element_to_bytes
    let new_len = (*e[0].field).to_bytes.as_ref().unwrap()(&mut buf[0] as *mut u8, &e[0]);
    buf.set_len(new_len as usize);
    buf
}

pub unsafe fn element_random(e: &mut element_t) {
    (*e[0].field).random.as_ref().unwrap()(&mut e[0])
}

pub unsafe fn element_is0(e: &element_t) -> bool {
    (*e[0].field).is0.as_ref().unwrap()(&e[0]) != 0
}

pub unsafe fn element_set0(e: &mut element_t) {
    (*e[0].field).set0.as_ref().unwrap()(&mut e[0])
}

pub unsafe fn pairing_apply(
    out: &mut element_t,
    in1: &element_t,
    in2: &element_t,
    pairing: &pairing_t,
) {
    assert_eq!(
        &pairing[0].GT[0] as *const _, out[0].field,
        "pairing output mismatch"
    );
    assert_eq!(
        pairing[0].G1 as *const _, in1[0].field,
        "pairing 1st input mismatch"
    );
    assert_eq!(
        pairing[0].G2 as *const _, in2[0].field,
        "pairing 2nd input mismatch"
    );
    if element_is0(in1) || element_is0(in2) {
        element_set0(out);
        return;
    }
    // TODO: 'out' is an element of a multiplicative subgroup, but the
    // pairing routine expects it to be an element of the full group, hence
    // the 'out->data'. I should make this clearer.
    pairing[0].map.as_ref().unwrap()(out[0].data as *mut _, &in1[0], &in2[0], &pairing[0]);
}
