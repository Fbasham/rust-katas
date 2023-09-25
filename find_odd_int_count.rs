use itertools::Itertools;

fn find_odd(a: &[i32]) -> i32 {
    **(a.iter().counts().iter().find(|(k,v)| **v%2==1 as usize).unwrap()).0
}