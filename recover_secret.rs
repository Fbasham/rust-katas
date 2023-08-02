use itertools::Itertools;

fn recover_secret(trip: Vec<[char; 3]>) -> String {
    let mut q = trip.iter().flatten().unique().join("");
    let mut v = trip.iter().map(|t| t.iter().join("")).collect::<Vec<_>>();
    let mut r = String::new();
    while q.len() > 0 {
        let x = q
            .chars()
            .find(|&e| {
                v.iter().all(|x| {
                    if !x.contains(e) {
                        true
                    } else {
                        &x[..1] == &e.to_string()
                    }
                })
            })
            .unwrap();
        v = v.iter().map(|e| e.replace(x, "")).collect();
        q = q.replace(x, "");
        r.push(x);
    }
    r
}
