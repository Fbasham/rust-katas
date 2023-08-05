fn circle(r: i32) -> String {
    if r<0 {return String::new()}
    let mut v = vec![];
    for i in 0..2*r-1 {
        let mut s = String::new();
        for j in 0..2*r-1 {
            s += if (i-(r-1)).pow(2)+(j-(r-1)).pow(2)<r*r {"█"} else {" "};
        }
        v.push(s);
    }
    v.join("\n")+"\n"
}   