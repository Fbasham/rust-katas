use std::collections::HashMap;

fn conjugate(verb: &str) -> HashMap<String, Vec<String>> {
    match &verb[verb.len() - 2..] {
        "er" => HashMap::from([(
            verb.to_string(),
            vec!["o", "es", "e", "emos", "éis", "en"]
                .iter()
                .map(|e| format!("{}{}", &verb[..verb.len() - 2], e))
                .collect::<Vec<_>>(),
        )]),
        "ar" => HashMap::from([(
            verb.to_string(),
            vec!["o", "as", "a", "amos", "áis", "an"]
                .iter()
                .map(|e| format!("{}{}", &verb[..verb.len() - 2], e))
                .collect::<Vec<_>>(),
        )]),
        _ => HashMap::from([(
            verb.to_string(),
            vec!["o", "es", "e", "imos", "ís", "en"]
                .iter()
                .map(|e| format!("{}{}", &verb[..verb.len() - 2], e))
                .collect::<Vec<_>>(),
        )]),
    }
}
