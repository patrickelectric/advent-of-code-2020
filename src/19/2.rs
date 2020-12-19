use aoc2020::helper;

mod messagechecker;
use messagechecker::MessageChecker;

fn main() {
    let content = helper::get_input_file("19-input.txt");
    let mut message_checker = MessageChecker::new();
    message_checker.process_input(&content);
    message_checker.update_rules_more(8, vec![42]);
    message_checker.update_rules_more(11, vec![42, 31]);
    helper::print_answer("19-1", message_checker.get_valid_messages(0).len() as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

        let mut message_checker = MessageChecker::new();
        message_checker.process_input(content);
        assert_eq!(message_checker.get_valid_messages(0).len(), 3);

        message_checker.update_rules_more(8, vec![42]);
        message_checker.update_rules_more(11, vec![42, 31]);
        assert_eq!(message_checker.get_valid_messages(0).len(), 12);
    }
}
