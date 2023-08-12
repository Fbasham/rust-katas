use std::collections::HashSet;

fn destroy(v: Vec<HashSet<char>>) -> String {
    let mut s = "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string();
    for x in v.iter() {
        for y in x.iter() {
            if y != &' ' {
                s = s.replacen(*y,"_",1);
            }
        }
    }
    s.to_string()
}