fn generate_shape(n: i32) -> String {
    (0..n)
        .map(|e| "+".repeat(n as usize))
        .collect::<Vec<String>>()
        .join("\n")
}
