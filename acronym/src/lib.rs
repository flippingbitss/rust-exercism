pub fn abbreviate(phrase: &str) -> String {
    let phrase: Vec<_> = phrase.chars().collect();
    let mut result: Vec<char> = Vec::new();

    if !phrase.is_empty() {
        result.push(*phrase.get(0).unwrap())
    }

    for i in 0..phrase.len() {
        let c = *phrase.get(i).unwrap();

        if let Some(&next) = phrase.get(i + 1) {
            if !c.is_alphanumeric() && next.is_alphanumeric() {
                result.push(next.to_ascii_uppercase());
            }

            if c.is_lowercase() && next.is_uppercase() {
                result.push(next);
            }
        }
    }

    result.iter().collect()
}