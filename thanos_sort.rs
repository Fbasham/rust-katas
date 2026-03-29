fn thanos_sort(arr: &[i32]) -> usize {
    if arr.len()==0 || arr.iter().zip(&arr[1..]).all(|(i,j)| i<=j) {return arr.len()}
    let k = arr.len();
    let a = &arr[..k/2+(k&1)];
    let b = &arr[k/2+(k&1)..];
    thanos_sort(a).max(thanos_sort(b))
}