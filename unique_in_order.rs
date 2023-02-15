fn unique_in_order<T>(seq: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut v = vec![];
    for e in seq {
        v.push(e)
    }
    v.dedup();
    v
}
