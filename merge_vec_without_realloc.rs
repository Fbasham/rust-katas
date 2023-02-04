fn merge<'a: 'c, 'b: 'c, 'c: 'a + 'b>(xs: &'a Vec<usize>, ys: &'b Vec<usize>) -> Vec<&'c usize> {
    xs.iter().chain(ys).collect()
}
