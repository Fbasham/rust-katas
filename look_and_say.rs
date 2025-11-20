fn look_say(n: u64) -> u64 {
    let mut r = String::new();
    let mut p = ' ';
    let mut k = 0;
    for e in format!("{n} ").chars() {
        if p==' ' || e==p {
            k += 1;
        }
        else {
            r = format!("{r}{k}{p}");
            p = e;
            k = 1;
        }
        p = e;
    }
    r.parse().unwrap()
}