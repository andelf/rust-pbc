use pbc::{Element, Pairing};
use std::fs;

pub fn main() {
    let param = fs::read_to_string("param/a.param").unwrap();
    let pairing: Pairing = param.parse().unwrap();

    if !pairing.is_symmetric() {
        panic!("pairing must be symmetric");
    }

    let mut p = Element::init_g1(&pairing);

    let mut a = Element::init_zr(&pairing);
    let mut b = Element::init_zr(&pairing);
    let mut c = Element::init_zr(&pairing);

    println!("Joux key agreement between A, B and C.");
    p.random();
    a.random();
    b.random();
    c.random();

    let t1 = Element::mul_zn(&p, &a);
    println!("A sends B and C: aP\naP = {}", t1);

    let t2 = Element::mul_zn(&p, &b);
    println!("B sends A and C: bP\nbP = {}", t2);

    let t3 = Element::mul_zn(&p, &c);
    println!("C sends A and B: cP\ncP = {}", t3);

    // 'out' = 'e'('in1', 'in2'),
    let t4 = Element::pairing(&t2, &t3);
    let ka = Element::pow_zn(&t4, &a);
    println!("Ka = {}", ka);

    let t5 = Element::pairing(&t1, &t3);
    let kb = Element::pow_zn(&t5, &b);
    // FIXME: lifetime hell
    // let kb = Element::pairing(&t1, &t3).pow(&b);
    println!("Kb = {}", kb);

    let t6 = Element::pairing(&t1, &t2);
    let kc = Element::pow_zn(&t6, &c);
    println!("Kc = {}", kc);

    println!("Shared key K = Ka = Kb = Kc");
}
