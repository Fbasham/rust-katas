fn arrange(s: &str) -> String {
    let mut v = s.split(" ").map(|e| e.to_string()).collect::<Vec<_>>();
    for i in 0..v.len()-1 {
        if i%2==0 {
            if v[i].len()>v[i+1].len(){
                (v[i],v[i+1]) = (v[i+1].clone(),v[i].clone())
            }
        }
        else {
            if v[i].len()<v[i+1].len(){
                (v[i],v[i+1]) = (v[i+1].clone(),v[i].clone())
            }
        }
    }
    v = v.iter().enumerate().map(|(i,e)| if i%2==1 {e.to_uppercase()} else {e.to_lowercase()}).collect();
    v.join(" ")
}