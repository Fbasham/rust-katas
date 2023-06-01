fn fizz_buzz_custom_solver(s1: &str, s2: &str, m: usize, n: usize) -> Vec<String> {
    (1..=100).map(|i| if i%m==0 && i%n==0 {s1.to_string()+s2} else if i%m==0 {s1.to_string()} else if i%n==0 {s2.to_string()} else {i.to_string()}).collect::<Vec<_>>()
}

#[macro_export]
macro_rules! fizz_buzz_custom {
    () => { fizz_buzz_custom_solver("Fizz", "Buzz", 3, 5) };
    ($str_one:expr) => { fizz_buzz_custom_solver($str_one, "Buzz", 3, 5) };
    ($str_one:expr, $str_two:expr) => { fizz_buzz_custom_solver($str_one, $str_two, 3, 5) };
    ($str_one:expr, $str_two:expr, $num_one:expr) => { fizz_buzz_custom_solver($str_one, $str_two, $num_one, 5) };
    ($str_one:expr, $str_two:expr, $num_one:expr, $num_two:expr) => { fizz_buzz_custom_solver($str_one, $str_two, $num_one, $num_two) };
}