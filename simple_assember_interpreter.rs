use std::collections::HashMap;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut d: HashMap<String, i64> = HashMap::new();
    let mut i = 0;
    while i < program.len() {
        let c = program[i];
        let mut x = c.split(" ").collect::<Vec<_>>();
        if x.len() < 3 {
            x.push("0")
        }
        let k = x[1].to_string();
        let v = if "-0123456789".contains(&x[2][..1]) {
            x[2].parse::<i64>().unwrap()
        } else {
            d.get(x[2]).unwrap().clone()
        };
        if c.starts_with("mov") {
            d.insert(k.to_string(), v);
        }
        if c.starts_with("inc") {
            d.insert(k.to_string(), d.get(&k).unwrap() + 1);
        }
        if c.starts_with("dec") {
            d.insert(k.to_string(), d.get(&k).unwrap() - 1);
        }
        if c.starts_with("jnz") && k != "0".to_string() && d.get(&k).unwrap_or(&-1) != &0 {
            i = (i as i64 + v) as usize;
            continue;
        }
        i += 1;
    }
    d
}
