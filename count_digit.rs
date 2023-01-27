fn nb_dig(n: i32, d: i32) -> i32 {
    (0..=n).map(|e| (e*e).to_string()).collect::<Vec<String>>().join("").chars().filter(|&e| e.to_string().parse::<i32>().unwrap()==d).count().try_into().unwrap()
}