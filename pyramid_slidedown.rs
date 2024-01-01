fn longest_slide_down(p: &[Vec<u16>]) -> u16 {
    let mut a = p.to_vec();
    while a.len()>1 {
        let t = a[a.len()-2].iter().enumerate().map(|(i,e)| (e+a[a.len()-1][i]).max(e+a[a.len()-1][i+1])).collect::<Vec<_>>();
        a.pop();
        a.pop();
        a.push(t);
    }
    a[0][0]
}