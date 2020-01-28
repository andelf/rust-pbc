#![allow(non_snake_case)]
use std::mem;
use std::os::raw::c_int;

pub use ffi::mpz_t;
pub use ffi::{element_t, field_s, pairing_t};

#[allow(dead_code, non_camel_case_types, unused_imports, non_snake_case)]
mod ffi;

#[link(name = "pbc")]
extern "C" {
    // 0 on success
    pub fn pairing_init_set_buf(pairing: &mut pairing_t, input: *const u8, len: usize) -> c_int;

    pub fn pairing_clear(pairing: &mut pairing_t);

    /// Returns the number of bytes needed to hold 'e' in compressed form.
    /// Currently only implemented for points on an elliptic curve.
    pub fn element_length_in_bytes_compressed(e: &element_t) -> c_int;
    /// If possible, outputs a compressed form of the element 'e' to
    /// the buffer of bytes 'data'.
    /// Currently only implemented for points on an elliptic curve.
    pub fn element_to_bytes_compressed(data: *mut u8, e: &element_t) -> c_int;

    /// Sets element 'e' to the element in compressed form in the buffer of bytes 'data'.
    /// Currently only implemented for points on an elliptic curve.
    pub fn element_from_bytes_compressed(e: &mut element_t, data: *const u8) -> c_int;

}

#[link(name = "gmp")]
extern "C" {
    fn __gmpz_init(_: &mut mpz_t);
    fn __gmpz_clear(_: &mut mpz_t);
}

// Initializing elements

unsafe fn element_init(e: &mut element_t, f: *const field_s) {
    e.field = f;
    (*f).init.as_ref().unwrap()(e)
}

pub unsafe fn element_init_G1(e: &mut element_t, pairing: &pairing_t) {
    element_init(e, pairing.G1);
}

pub unsafe fn element_init_G2(e: &mut element_t, pairing: &pairing_t) {
    element_init(e, pairing.G2);
}

pub unsafe fn element_init_GT(e: &mut element_t, pairing: &pairing_t) {
    element_init(e, &pairing.GT);
}

pub unsafe fn element_init_Zr(e: &mut element_t, pairing: &pairing_t) {
    element_init(e, &pairing.Zr);
}

/// Initialize e to be an element of the algebraic structure that e2 lies in.
pub unsafe fn element_init_same_as(e: &mut element_t, e2: &element_t) {
    element_init(e, e2.field);
}

pub unsafe fn element_clear(e: &mut element_t) {
    (*e.field).clear.as_ref().unwrap()(e)
}

// Assigning elements

pub unsafe fn element_set0(e: &mut element_t) {
    (*e.field).set0.as_ref().unwrap()(e)
}

pub unsafe fn element_set1(e: &mut element_t) {
    (*e.field).set1.as_ref().unwrap()(e)
}

pub unsafe fn element_set_i64(e: &mut element_t, i: i64) {
    (*e.field).set_si.as_ref().unwrap()(e, i)
}

pub unsafe fn element_set(e: &mut element_t, a: &element_t) {
    (*e.field).set.as_ref().unwrap()(e, a)
}

// Converting elements

/// Converts 'e' to a GMP integer 'z'
/// if such an operation makes sense
pub unsafe fn element_to_mpz(z: &mut mpz_t, e: &element_t) {
    (*e.field).to_mpz.as_ref().unwrap()(z, e);
}

pub unsafe fn element_from_hash(e: &mut element_t, data: &[u8]) {
    (*e.field).from_hash.as_ref().unwrap()(e, data.as_ptr() as _, data.len() as _);
}

// Element arithmetic

pub unsafe fn element_add(n: &mut element_t, a: &element_t, b: &element_t) {
    // PBC_ASSERT_MATCH3(n, a, b);
    (*n.field).add.as_ref().unwrap()(n, a, b)
}

pub unsafe fn element_sub(n: &mut element_t, a: &element_t, b: &element_t) {
    // PBC_ASSERT_MATCH3(n, a, b);
    (*n.field).sub.as_ref().unwrap()(n, a, b)
}

pub unsafe fn element_mul(n: &mut element_t, a: &element_t, b: &element_t) {
    // PBC_ASSERT_MATCH3(n, a, b);
    (*n.field).mul.as_ref().unwrap()(n, a, b)
}

pub unsafe fn element_mul_mpz(n: &mut element_t, a: &element_t, z: &mpz_t) {
    // PBC_ASSERT_MATCH2(n, a);
    (*n.field).mul_mpz.as_ref().unwrap()(n, a, z)
}

/// 'z' must be an element of a integer mod ring (i.e. *Z*~n~ for some n).
/// Set 'c' = 'a' 'z', that is 'a' + 'a' + ... + 'a'
/// where there are 'z' 'a''s.
pub unsafe fn element_mul_zn(c: &mut element_t, a: &element_t, z: &element_t) {
    let mut z0: mpz_t = mem::zeroed();
    // PBC_ASSERT_MATCH2(c, a);
    // TODO: check z->field is Zn
    __gmpz_init(&mut z0);
    element_to_mpz(&mut z0, z);
    element_mul_mpz(c, a, &z0);
    __gmpz_clear(&mut z0);
}

pub unsafe fn element_div(n: &mut element_t, a: &element_t, b: &element_t) {
    // PBC_ASSERT_MATCH3(n, a, b);
    (*n.field).div.as_ref().unwrap()(n, a, b)
}

pub unsafe fn element_double(n: &mut element_t, a: &element_t) {
    // PBC_ASSERT_MATCH2(n, a);
    (*n.field).doub.as_ref().unwrap()(n, a)
}

pub unsafe fn element_halve(n: &mut element_t, a: &element_t) {
    // PBC_ASSERT_MATCH2(n, a);
    (*n.field).halve.as_ref().unwrap()(n, a)
}

pub unsafe fn element_square(n: &mut element_t, a: &element_t) {
    // PBC_ASSERT_MATCH2(n, a);
    (*n.field).square.as_ref().unwrap()(n, a)
}

pub unsafe fn element_neg(n: &mut element_t, a: &element_t) {
    // PBC_ASSERT_MATCH2(n, a);
    (*n.field).neg.as_ref().unwrap()(n, a)
}

pub unsafe fn element_invert(n: &mut element_t, a: &element_t) {
    // PBC_ASSERT_MATCH2(n, a);
    (*n.field).invert.as_ref().unwrap()(n, a)
}

// Exponentiating elements

pub unsafe fn element_pow_mpz(x: &mut element_t, a: &element_t, n: &mpz_t) {
    // PBC_ASSERT_MATCH2(x, a)
    (*x.field).pow_mpz.as_ref().unwrap()(x, a, n);
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

// Comparing elements

pub unsafe fn element_is0(e: &element_t) -> bool {
    (*e.field).is0.as_ref().unwrap()(e) != 0
}

pub unsafe fn element_is1(e: &element_t) -> bool {
    (*e.field).is1.as_ref().unwrap()(e) != 0
}

pub unsafe fn element_cmp(a: &element_t, b: &element_t) -> i32 {
    // PBC_ASSERT_MATCH2(a, b)
    (*a.field).cmp.as_ref().unwrap()(a, b)
}

// Random elements

pub unsafe fn element_random(e: &mut element_t) {
    (*e.field).random.as_ref().unwrap()(e)
}

// Element import/export

pub unsafe fn element_snprint(s: *mut u8, n: usize, e: &element_t) -> c_int {
    (*e.field).snprint.as_ref().unwrap()(s as _, n, e)
}

pub unsafe fn element_length_in_bytes(e: &element_t) -> i32 {
    if (*e.field).fixed_length_in_bytes < 0 {
        return (*e.field).length_in_bytes.as_ref().unwrap()(e);
    }
    return (*e.field).fixed_length_in_bytes;
}

pub unsafe fn element_to_bytes(e: &element_t) -> Vec<u8> {
    let len = element_length_in_bytes(e);
    let mut buf = vec![0u8; len as usize];
    // element_to_bytes
    let new_len = (*e.field).to_bytes.as_ref().unwrap()(&mut buf[0] as *mut u8, e);
    buf.set_len(new_len as usize);
    buf
}

// pairing_apply

pub unsafe fn pairing_apply(
    out: &mut element_t,
    in1: &element_t,
    in2: &element_t,
    pairing: &pairing_t,
) {
    assert_eq!(
        &pairing.GT as *const _, out.field,
        "pairing output mismatch"
    );
    assert_eq!(
        pairing.G1 as *const _, in1.field,
        "pairing 1st input mismatch"
    );
    assert_eq!(
        pairing.G2 as *const _, in2.field,
        "pairing 2nd input mismatch"
    );
    if element_is0(in1) || element_is0(in2) {
        element_set0(out);
        return;
    }
    // TODO: 'out' is an element of a multiplicative subgroup, but the
    // pairing routine expects it to be an element of the full group, hence
    // the 'out->data'. I should make this clearer.
    pairing.map.as_ref().unwrap()(out.data as *mut _, in1, in2, pairing);
}

// pairing_op
pub unsafe fn pairing_is_symmetric(pairing: &pairing_t) -> bool {
    return pairing.G1 == pairing.G2
}
