pub use pairing::Pairing;
pub use element::Element;

mod pairing;
mod element;

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_hello_world() {
        let param = fs::read_to_string("../param/a.param").unwrap();
        let pairing: Pairing = param.parse().unwrap();
        // println!("got => {:?}", pairing);
        let mut g = Element::init_g2(&pairing);
        let mut h = Element::init_g1(&pairing);

        let mut temp1 = Element::init_gt(&pairing);
        let mut temp2 = Element::init_gt(&pairing);

        let mut secret_key = Element::init_zr(&pairing);

        // generate system parameters
        g.random();

        // generate a private key
        secret_key.random();

        let public_key = Element::pow_zn(&g, &secret_key);
        h.from_hash(b"ABCDEF");

        let sig = Element::pow_zn(&h, &secret_key);

        pairing.apply(&mut temp1, &sig, &g);
        pairing.apply(&mut temp2, &h, &public_key);

        if temp1 == temp2 {
            println!("ok, \n{:?}\n{:?}", temp1, temp2);
        }


        println!("g => {:?}", g);
    }
}
