fn to_csv_text(a: &[Vec<i8>]) -> String {
    a.iter()
        .map(|t| {
            t.iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(",")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
