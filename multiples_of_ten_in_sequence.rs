use num::bigint::*;

fn find_multiples_of_10_sf(n: u32) -> BigUint {
    let mut r = 1.to_biguint().unwrap();
    let mut k = 0;
    for _ in 0..n {
        for _ in k.. {
            k += 1;
            r = (6.to_biguint().unwrap().pow(k)
                + 3.to_biguint().unwrap() * 2.to_biguint().unwrap().pow(k))
                / 4.to_biguint().unwrap();
            if r.clone() % 10.to_biguint().unwrap() == 0.to_biguint().unwrap() {
                break;
            }
        }
    }
    r
}
