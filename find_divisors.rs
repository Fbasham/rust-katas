fn divisors(n: u32) -> Result<Vec<u32>, String> {
    let mut v = vec![];
    for i in 2..n {
        if n % i == 0 {
            v.push(i);
        }
    }
    match v.len() > 0 {
        true => Result::Ok(v),
        false => Result::Err(format!("{n} is prime")),
    }
}
