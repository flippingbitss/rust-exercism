////iterators
//pub fn build_proverb(list: &[&str]) -> String {
//    use std::iter::once;
//
//    if let Some(start) = list.first() {
//        return list
//            .windows(2)
//            .map(|x| match x {
//                &[a, b] => String::from(format!("For want of a {} the {} was lost.\n", a, b)),
//                _ => unreachable!(),
//            })
//            .chain(once(String::from(format!("And all for the want of a {}.", start))))
//            .collect::<Vec<_>>()
//            .join("");
//    }
//
//    String::new()
//}

use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let last_line = format!("And all for the want of a {}.", list[0]);
    list.windows(2)
        .map(|win| format!("For want of a {0} the {1} was lost.", win[0], win[1]))
        .chain(once(last_line))
        .collect::<Vec<String>>()
        .join("\n")
}

//imperative method
//pub fn build_proverb(list: &[&str]) -> String {
//    if let Some(start) = list.first() {
//        let mut result = Vec::new();
//
//        for pairs in list.windows(2) {
//            match pairs {
//                [a, b] => result.push(String::from(format!("For want of a {} the {} was lost.\n", a, b))),
//                _ => unreachable!(),
//            };
//        }
//
//        result.push(String::from(format!(
//            "And all for the want of a {}.",
//            start
//        )));
//
//        return result.join("");
//    };
//
//    String::new()
//}