use itertools::Itertools;

fn rolldice_sum_prob(s:i32, k:i32) -> f64 {
    let mut c = 0;
    for p in (0..k).map(|i| (1..=6)).multi_cartesian_product() {
        if p.iter().sum::<i32>()==s {
            c += 1;
        }
    }
    c as f64/6.0_f64.powf(k as f64)
}