use hex::ToHex;
use pbc_sys::*;
use std::cmp;
use std::fmt;
use std::mem;

use crate::pairing::Pairing;

pub struct Element<'a> {
    pub(crate) inner: element_t,
    pub(crate) pairing: &'a Pairing,
}

impl fmt::Debug for Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = unsafe { element_to_bytes(&self.inner) };
        write!(f, "{}", raw.encode_hex::<String>())
    }
}

impl fmt::Display for Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 1024];
        let len = unsafe {
            element_snprint(buf.as_mut_ptr(), 1024, &self.inner) as usize
        };
        write!(f, "{}", String::from_utf8_lossy(&buf[..len]))
    }
}

impl Drop for Element<'_> {
    fn drop(&mut self) {
        unsafe { element_clear(&mut self.inner) }
    }
}

impl PartialEq for Element<'_> {
    fn eq(&self, other: &Element<'_>) -> bool {
        self.partial_cmp(other) == Some(cmp::Ordering::Equal)
    }
}

impl PartialOrd for Element<'_> {
    fn partial_cmp(&self, other: &Element<'_>) -> Option<cmp::Ordering> {
        let cmp = unsafe { element_cmp(&self.inner, &other.inner) };
        match cmp {
            0 => Some(cmp::Ordering::Equal),
            1 => Some(cmp::Ordering::Greater),
            -1 => Some(cmp::Ordering::Less),
            _ => unreachable!(),
        }
    }
}

impl<'a> Element<'a> {
    pub fn init_g1<'b: 'a>(pairing: &'b Pairing) -> Self {
        let mut a = unsafe { mem::zeroed() };
        unsafe { element_init_G1(&mut a, &pairing.0) };
        Element { inner: a, pairing }
    }

    pub fn init_g2<'b: 'a>(pairing: &'b Pairing) -> Self {
        let mut a = unsafe { mem::zeroed() };
        unsafe { element_init_G2(&mut a, &pairing.0) };
        Element { inner: a, pairing }
    }

    pub fn init_gt<'b: 'a>(pairing: &'b Pairing) -> Self {
        let mut a = unsafe { mem::zeroed() };
        unsafe { element_init_GT(&mut a, &pairing.0) };
        Element { inner: a, pairing }
    }

    pub fn init_zr<'b: 'a>(pairing: &'b Pairing) -> Self {
        let mut a = unsafe { mem::zeroed() };
        unsafe { element_init_Zr(&mut a, &pairing.0) };
        Element { inner: a, pairing }
    }

    pub fn init_same_as<'b: 'a>(e: &'b Element<'b>) -> Self {
        let mut a = unsafe { mem::zeroed() };
        unsafe { element_init_same_as(&mut a, &e.inner) };
        Element {
            inner: a,
            pairing: e.pairing,
        }
    }

    pub fn random(&mut self) {
        unsafe { element_random(&mut self.inner) }
    }

    pub fn pow_zn<'b: 'a>(a: &'b Element, n: &'b Element) -> Self {
        let mut x = Element::init_same_as(a);
        unsafe {
            element_pow_zn(&mut x.inner, &a.inner, &n.inner);
        }
        x
    }

    pub fn mul_zn<'b: 'a>(a: &'b Element, z: &'b Element) -> Self {
        let mut c = Element::init_same_as(a);
        unsafe {
            element_mul_zn(&mut c.inner, &a.inner, &z.inner);
        }
        c
    }

    pub fn from_hash(&mut self, data: &[u8]) {
        unsafe { element_from_hash(&mut self.inner, data) }
    }

    pub fn pairing<'b: 'a>(in1: &'b Element, in2: &'b Element) -> Self {
        let mut out = Element::init_gt(&in1.pairing);
        out.pairing.apply(&mut out, in1, in2);
        out
    }
}
