use core::f64::consts::PI;
use either::*;

fn sort_by_area(a: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let f = |e: &Either<(f64, f64), f64>| match e {
        Left(x) => x.0 * x.1,
        Right(x) => PI * x * x,
    };
    let mut v = a.to_vec();
    v.sort_by(|a, b| f(a).partial_cmp(&f(b)).unwrap());
    v
}
