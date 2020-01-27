use pbc_sys::*;
use std::io;
use std::mem;
use std::str::FromStr;

use crate::element::Element;

#[derive(Debug)]
pub struct Pairing(pub(crate) pairing_t);

impl FromStr for Pairing {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairing: pairing_t = unsafe { mem::zeroed() };
        if unsafe { pairing_init_set_buf(&mut pairing, s.as_bytes().as_ptr(), s.len() as _) } == 0 {
            Ok(Pairing(pairing))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "can not pairing_init_set_buf",
            ))
        }
    }
}

impl Drop for Pairing {
    fn drop(&mut self) {
        unsafe { pairing_clear(&mut self.0) }
    }
}

impl Pairing {
    pub fn apply(&self, out: &mut Element, in1: &Element, in2: &Element) {
        unsafe { pairing_apply(&mut out.inner, &in1.inner, &in2.inner, &self.0) }
    }
}
