use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Rule {
    Character(char),
    Redirect(Vec<Vec<u64>>),
}

#[derive(Debug)]
pub struct MessageChecker {
    rules: HashMap<u64, Rule>,
    more_rules: HashMap<u64, Vec<u64>>,
    message: String,
}

impl MessageChecker {
    pub fn new() -> Self {
        return Self {
            rules: HashMap::new(),
            more_rules: HashMap::new(),
            message: String::new(),
        };
    }

    pub fn process_input(&mut self, input: &str) {
        let mut content = input.split("\n\n");
        let rules = content.next().unwrap();
        self.message = content.next().unwrap().into();

        for line in rules.lines() {
            self.update_rules(line);
        }
    }

    pub fn update_rules_more(&mut self, id: u64, ids: Vec<u64>) {
        self.more_rules.insert(id, ids);
    }

    pub fn update_rules(&mut self, rule_string: &str) {
        let simple_rule = Regex::new(r#"(?P<id>\d+): "(?P<character>\S)""#).unwrap();
        let redirect_rule = Regex::new(r#"(?P<id>\d+): (?P<redirect>[\d\| ]+)"#).unwrap();

        if let Some(redirect) = redirect_rule.captures(rule_string) {
            let id = redirect
                .name("id")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let redirect_rules = redirect.name("redirect").unwrap().as_str();
            let mut rules_options: Vec<Vec<u64>> = vec![];
            for order in redirect_rules.split("|") {
                let mut rules_option: Vec<u64> = vec![];
                for rule_id in order.split(" ").filter(|x| !x.is_empty()) {
                    let rule_id = rule_id.parse::<u64>().unwrap();
                    rules_option.push(rule_id);
                }
                rules_options.push(rules_option);
            }
            let rule = Rule::Redirect(rules_options);
            self.rules.insert(id, rule);
        } else if let Some(simple) = simple_rule.captures(rule_string) {
            let id = simple.name("id").unwrap().as_str().parse::<u64>().unwrap();
            let character = simple
                .name("character")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();
            let rule = Rule::Character(character);
            self.rules.insert(id, rule);
        } else {
            unreachable!();
        }
    }

    pub fn get_valid_messages(&self, id: u64) -> Vec<String> {
        let regex_string = format!("^{}$", self.generate_regex(id));
        let mut messages = vec![];
        for i in 1..=23 {
            let regex = Regex::new(&regex_string.replace("n", &i.to_string())).unwrap();
            messages.append(
                &mut self
                    .message
                    .lines()
                    .filter(|line| regex.is_match(line))
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>(),
            );
        }
        messages.sort();
        messages.dedup();
        return messages;
    }

    pub fn generate_regex(&self, id: u64) -> String {
        match &self.rules[&id] {
            Rule::Character(character) => {
                let character = character.to_string();
                return character;
            }
            Rule::Redirect(redirect) => {
                let mut answers = vec![];
                for options in redirect {
                    let mut answer = vec![];
                    for option in options {
                        answer.push(self.generate_regex(*option));
                        if self.more_rules.contains_key(&id) {
                            if self.more_rules[&id].contains(option) {
                                if self.more_rules[&id].len() == 1 {
                                    answer.push("+".to_string());
                                } else {
                                    answer.push("{n}".to_string());
                                }
                            }
                        }
                    }
                    answers.push(answer.join(""));
                }
                return format!("({})", answers.join("|"));
            }
        }
    }
}
