fn solution(a: &[Vec<char>]) -> bool {
    let t = a.iter().find(|e| e.contains(&'>')).unwrap();
    let i = t.iter().position(|e| e == &'>').unwrap();
    let j = t.iter().position(|e| e == &'x').unwrap_or(0);
    i < j
}
