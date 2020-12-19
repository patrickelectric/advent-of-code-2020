use aoc2020::helper;

mod messagechecker;
use messagechecker::MessageChecker;

fn main() {
    let content = helper::get_input_file("19-input.txt");
    let mut message_checker = MessageChecker::new();
    message_checker.process_input(&content);
    helper::print_answer("19-1", message_checker.get_valid_messages(0).len() as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

        let mut message_checker = MessageChecker::new();
        message_checker.process_input(content);
        assert_eq!(message_checker.get_valid_messages(0).len(), 2);
    }
}
