fn binary_slice_to_number(a: &[u32]) -> u32 {
    u32::from_str_radix(
        &a.iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(""),
        2,
    )
    .unwrap()
}
