fn html_special_chars(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace(">", "&gt;")
        .replace("<", "&lt;")
        .replace("\"", "&quot;")
}
