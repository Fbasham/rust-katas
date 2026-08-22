fn wrap(h: u32, w: u32, l: u32) -> u32{
    let mut v = vec![h,w,l];
    v.sort();
    4*v[0]+2*v[1]+2*v[2]+20
}