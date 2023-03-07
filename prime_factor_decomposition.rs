use itertools::Itertools;

fn prime_factors(mut n: i64) -> String {
    let mut v = vec![];
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            v.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        v.push(n);
    }
    v.iter()
        .counts()
        .iter()
        .sorted()
        .map(|(i, j)| match j {
            1 => format!("({})", i),
            _ => format!("({}**{})", i, j),
        })
        .collect::<String>()
}
