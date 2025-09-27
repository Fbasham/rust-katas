fn card_game(mut n: u64) -> u64 {
    let (mut a, mut _b, mut k) = (0,0,0);
    while n>0 {
        let c = match n%2 {
            0 if n==4 => 2,
            0 => if (n/2)%2==0 {1} else {n/2},
            _ => 1
        };
        match k%2 {
            0 => a += c,
            _ => _b += c,
        }
        n -= c;
        k ^= 1;
    }
    a
}