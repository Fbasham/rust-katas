mod preloaded;
use preloaded::NATO;

fn to_nato(s: &str) -> String {
    s.to_uppercase().replace(" ","").chars().map(|e| NATO.get(&e).unwrap_or(&e.to_string().as_str()).to_string()+" ").collect::<String>().trim().to_string()
}