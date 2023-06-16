use num::BigUint;

fn all_fibonacci_numbers() -> impl Iterator<Item = BigUint> {
    let mut a = "1".parse::<BigUint>().unwrap();
    let mut b = "1".parse::<BigUint>().unwrap();
    (0..).map(move |_| {
        let r = a.clone();
        (a, b) = (b.clone(), a.clone() + b.clone());
        r
    })
}
