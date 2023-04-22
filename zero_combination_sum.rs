use either::Either;
use itertools::Itertools;

fn find_zero_sum_groups(a: &[i32], n: u32) -> Either<&'static str, Vec<Vec<i32>>> {
    if a.len()==0 {return Either::Left("No elements to combine")}
    let v = a.iter().cloned().unique().sorted().combinations(n as usize).filter(|t| t.iter().sum::<i32>()==0).collect::<Vec<_>>();
    if v.len()==0 {Either::Left("No combinations")} else {Either::Right(v)}
}