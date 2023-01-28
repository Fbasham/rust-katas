fn add_letters(v: Vec<char>) -> char {
    if v.is_empty() {
        return 'z';
    }
    char::from((v.iter().map(|e| (*e as u8) - 96).sum::<u8>() - 1) % 26 + 97)
}
