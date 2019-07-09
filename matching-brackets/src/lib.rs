use std::collections::vec_deque::VecDeque;

fn inverse_bracket(c: char) -> char {
    match c {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        _ => '_',
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .fold(VecDeque::new(), |mut acc, c| {
            match c {
                '[' | '{' | '(' => acc.push_back(c),
                ']' | '}' | ')' => {
                    if let Some(&last) = acc.back() {
                        if inverse_bracket(last) == c {
                            acc.pop_back();
                            return acc;
                        }
                    }
                    acc.push_back(c);
                }
                _ => {}
            }
            acc
        })
        .is_empty()
}
