pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let alphabets: Vec<_> = message.chars().filter(|&c| c.is_alphabetic()).collect();
    let digits: Vec<_> = message.chars().filter(|&c| c.is_digit(10)).collect();
    let has_alphabets = !alphabets.is_empty();
    let has_digits = !digits.is_empty();

    let is_empty = message.is_empty();
    let is_question = message.ends_with("?");
    let is_yell = has_alphabets && alphabets.iter().all(|&c| c.is_uppercase());

    if is_empty || !is_question && !has_alphabets && !has_digits {
        return "Fine. Be that way!";
    }

    match (is_yell, is_question) {
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
