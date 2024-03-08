use preloaded::{Token, TokenKind};
use regex::Regex;

struct Simplexer {
    a: Vec<String>,
}

impl Simplexer {
    fn new(s: &str) -> Self {
        Simplexer {
            a: Regex::new(r#"("(.|\n)*?")|([_\$a-zA-Z][\w$]*)|(true|false)|(if|else|for|while|return|func|break)|(\d+)|([+\-*/%()=])|(\s+)"#).unwrap().find_iter(s).map(|e| e.as_str().to_string()).collect::<Vec<String>>()
        }
    }
}

impl Iterator for Simplexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.a.len() < 1 {
            return None;
        }
        let x = self.a.remove(0);
        if Regex::new(r#"^("(.|\n)*?")$"#).unwrap().is_match(&x) {
            return Some(Token::new(&x, TokenKind::String));
        }
        if Regex::new(r"^(true|false)$").unwrap().is_match(&x) {
            return Some(Token::new(&x, TokenKind::Boolean));
        }
        if Regex::new(r"^(if|else|for|while|return|func|break)$")
            .unwrap()
            .is_match(&x)
        {
            return Some(Token::new(&x, TokenKind::Keyword));
        }
        if Regex::new(r"^([_\$a-zA-Z][\w$]*)$").unwrap().is_match(&x) {
            return Some(Token::new(&x, TokenKind::Identifier));
        }
        if Regex::new(r"^(\d+)$").unwrap().is_match(&x) {
            return Some(Token::new(&x, TokenKind::Integer));
        }
        if Regex::new(r"^([+\-*/%()=])$").unwrap().is_match(&x) {
            return Some(Token::new(&x, TokenKind::Operator));
        }
        if Regex::new(r"^(\s+)$").unwrap().is_match(&x) {
            return Some(Token::new(&x, TokenKind::Whitespace));
        }
        None
    }
}
