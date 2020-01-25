#![allow(non_snake_case)]
use hex::ToHex;
use std::fs::{self};
use std::mem;
use pbc_sys::*;

pub fn main() {
    let param = fs::read_to_string("param/e.param").unwrap();
    let mut pairing: pairing_t = unsafe { mem::zeroed() };

    let mut g: element_t = unsafe { mem::zeroed() };
    let mut h: element_t = unsafe { mem::zeroed() };

    let mut public_key: element_t = unsafe { mem::zeroed() };
    let mut secret_key: element_t = unsafe { mem::zeroed() };

    let mut sig: element_t = unsafe { mem::zeroed() };

    let mut temp1: element_t = unsafe { mem::zeroed() };
    let mut temp2: element_t = unsafe { mem::zeroed() };

    unsafe {
        let ret = pairing_init_set_buf(
            &mut pairing,
            &param.as_bytes()[0] as *const _ as _,
            param.len(),
        );
        println!("ret => {}", ret);

        element_init_G2(&mut g, &pairing);
        element_init_G2(&mut public_key, &pairing);
        element_init_G1(&mut h, &pairing);
        element_init_G1(&mut sig, &pairing);
        element_init_GT(&mut temp1, &pairing);
        element_init_GT(&mut temp2, &pairing);
        element_init_Zr(&mut secret_key, &pairing);

        // generate system parameters,
        element_random(&mut g);

        // generate a private key,
        element_random(&mut secret_key);

        // and the corresponding public key.
        element_pow_zn(&mut public_key, &g, &secret_key);

        // Say the message hash is "ABCDEF" (a 48-bit hash). We map these bytes to an element h of G1,
        element_from_hash(&mut h, b"ABCDEF");

        // then sign it:
        element_pow_zn(&mut sig, &h, &secret_key);

        // To verify this signature
        pairing_apply(&mut temp1, &sig, &g, &pairing);
        pairing_apply(&mut temp2, &h, &public_key, &pairing);

        if element_cmp(&temp1, &temp2) == 0 {
            println!("signature verifies");
        } else {
            println!("signature does not verify");
        }

        let len = element_length_in_bytes(&g);
        println!("len in bytes => {}", len);

        let x = element_to_bytes(&g);
        println!("g => {:}", x.encode_hex::<String>());
        println!(
            "public_key => {:}",
            element_to_bytes(&public_key).encode_hex::<String>()
        );
        println!("h => {:}", element_to_bytes(&h).encode_hex::<String>());
        println!("sig => {:}", element_to_bytes(&sig).encode_hex::<String>());

        println!(
            "temp1 => {:}",
            element_to_bytes(&temp1).encode_hex::<String>()
        );
        println!(
            "temp2 => {:}",
            element_to_bytes(&temp2).encode_hex::<String>()
        );
    }
}
