use regex::Regex;

fn brainfuck_to_c(input: &str) -> Result<String, &'static str> {
    let mut s = input.to_string();
    s = Regex::new("[^\\+-<>,\\.\\[\\]]").unwrap().replace_all(&s,"").to_string();
    
    let re = Regex::new("\\+-|-\\+|\\[\\]|<>|><").unwrap();
    while re.is_match(&s) {
        s = re.replace_all(&s,"").to_string();
    }
    
    let mut d = 0;
    for e in s.chars() {
        d += if e=='[' {1} else if e==']' {-1} else {0};
        if d<0 {break}
    }
    if d != 0 {return Err("")}
    
    let mut r = String::new();
    let mut k = 0;
    for m in Regex::new("\\++|-+|>+|<+|\\[|\\]|\\.|,").unwrap().find_iter(&s) {      
        let e = m.as_str();
        if "+-".contains(&e[..1]){   
            r += &format!("{}*p {}= {};\n"," ".repeat(k),&e[..1],e.len());
        }
        if "<>".contains(&e[..1]){   
            r += &format!("{}p {}= {};\n"," ".repeat(k),if &e[..1]==">" {"+"} else {"-"},e.len());
        }
        if e=="." {   
            r += &format!("{}putchar(*p);\n"," ".repeat(k));
        }
        if e=="," {
            r += &format!("{}*p = getchar();\n"," ".repeat(k));
        }
        if e=="[" {   
            r += &format!("{}if (*p) do {{\n"," ".repeat(k));
            k += 2;
        }
        if e=="]" {   
            k -= 2;
            r += &format!("{}}} while (*p);\n"," ".repeat(k));
        }
    }
    
    return Ok(r)
}