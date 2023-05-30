use itertools::Itertools;

fn de_nico(key: &str, msg: &str) -> String {
    let k = key.chars().enumerate().sorted_by_key(|t| t.1).map(|t| t.0).collect::<Vec<_>>();
    let mut v = vec![String::new();key.len()];
    msg.chars().enumerate().for_each(|t| v[t.0%k.len()] += (t.1).to_string().as_str());
    let m = v.iter().map(|e| e.len()).max().unwrap();
    v = k.iter().zip(v.iter().map(|e| e.to_string()+&" ".repeat(m-e.len()))).sorted().map(|t| t.1).collect::<Vec<_>>();
    v[0].chars().enumerate().map(|t| v.iter().map(|x| x.chars().nth(t.0).unwrap_or(' ')).collect::<String>()).collect::<String>().replace(" ","").to_string()
}