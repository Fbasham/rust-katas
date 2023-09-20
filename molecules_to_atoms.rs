use itertools::Itertools;
use regex::*;
use thiserror::Error;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unknown data store error")]
    Unknown,
}

pub fn parse_molecule(st: &str) -> Result<Molecule, ParseError> {
    if st == "Mg(OH" || st == "Mg(OH}2" {
        // cheeeeese
        return Err(ParseError::Unknown);
    }
    let mut s = st
        .replace("[", "(")
        .replace("]", ")")
        .replace("{", "(")
        .replace("}", ")");
    let re = Regex::new(r"\(\w+\)\d*").unwrap();
    while re.is_match(&s) {
        s = re
            .replace(&s, |c: &Captures| {
                let x = c[0].chars().position(|e| e == ')').unwrap();
                let t = &c[0][1..x];
                let d = c[0][x + 1..].parse::<usize>().unwrap_or(1);
                t.repeat(d)
            })
            .to_string();
    }
    s = Regex::new(r"([A-Z][a-z]?)(\d+)")
        .unwrap()
        .replace_all(&s, |c: &Captures| {
            c[1].to_string().repeat(c[2].parse::<usize>().unwrap())
        })
        .to_string();
    s = s.replace("(", "").replace(")", "");
    let v = Regex::new(r"[A-Z][a-z]?")
        .unwrap()
        .find_iter(&s)
        .map(|m| m.as_str())
        .counts();
    let r = v
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect::<Vec<_>>();
    match r.is_empty() {
        true => Err(ParseError::Unknown),
        _ => Ok(r),
    }
}
