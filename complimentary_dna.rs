fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|e| match e {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => e,
        })
        .collect()
}
