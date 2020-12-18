use aoc2020::helper;

use regex::Regex;

/* Parser group */
#[derive(Debug, Clone, Copy)]
pub enum MathType {
    Number(i64),
    Parentheses,
    Sum,
    Product,
}

#[derive(Debug)]
pub enum ParserNode {
    Nested(Vec<ParserNode>),
    Entry(MathType),
}

/* Lexical analyzer */
#[derive(Debug)]
pub enum LexicalType {
    Number(i64),
    Operation(char),
    Parentheses(char),
}

pub fn lexical_analyzer_run(string_equation: &str) -> Vec<LexicalType> {
    let mut result = vec![];

    // One may ask, why am I using regex ?, well that makes it easier to parse, not fast but this does not need to be
    let re = Regex::new(r"(?P<number>\d+)|(?P<operation>\+|\*)|(?P<parentheses>\(|\))").unwrap();
    for capture in re.captures_iter(string_equation) {
        if let Some(number) = capture.name("number") {
            let number = number.as_str().parse::<i64>().unwrap();
            result.push(LexicalType::Number(number));
        }
        if let Some(operation) = capture.name("operation") {
            let operation = operation.as_str().chars().next().unwrap();
            result.push(LexicalType::Operation(operation));
        }
        if let Some(parentheses) = capture.name("parentheses") {
            let parentheses = parentheses.as_str().chars().next().unwrap();
            result.push(LexicalType::Parentheses(parentheses));
        }
    }

    return result;
}

pub fn parser_run_iter(lexical_iter: &mut dyn Iterator<Item = &LexicalType>) -> ParserNode {
    let mut nodes = vec![];
    let mut maybe_lexical_node = lexical_iter.next();

    while maybe_lexical_node.is_some() {
        match maybe_lexical_node.unwrap() {
            LexicalType::Number(number) => nodes.push(ParserNode::Entry(MathType::Number(*number))),
            LexicalType::Operation(operation) => match operation {
                '*' => nodes.push(ParserNode::Entry(MathType::Product)),
                '+' => nodes.push(ParserNode::Entry(MathType::Sum)),
                _ => unreachable!("wth"),
            },
            LexicalType::Parentheses(parentheses) => match parentheses {
                '(' => nodes.push(parser_run_iter(lexical_iter)),
                ')' => {
                    return ParserNode::Nested(nodes);
                }
                _ => unreachable!("wth"),
            },
        }
        maybe_lexical_node = lexical_iter.next();
    }

    return ParserNode::Nested(nodes);
}

pub fn parser_run(lexical_result: &Vec<LexicalType>) -> ParserNode {
    let mut lexical_iter = lexical_result.iter();
    return parser_run_iter(&mut lexical_iter);
}

pub fn solve_node_iter(node_iter: &mut dyn Iterator<Item = &ParserNode>) -> i64 {
    let mut maybe_node_iter = node_iter.next();

    let mut left_number: i64 = 0;
    let mut operation: Option<MathType> = None;

    while maybe_node_iter.is_some() {
        match maybe_node_iter.unwrap() {
            ParserNode::Entry(entry) => match entry {
                MathType::Number(number) => match operation {
                    Some(op) => {
                        match op {
                            MathType::Product => left_number *= number,
                            MathType::Sum => left_number += number,
                            _ => unreachable!(),
                        };
                        operation = None;
                    }
                    None => left_number = *number,
                },
                _ => operation = Some(*entry),
            },
            ParserNode::Nested(nested) => {
                let mut nested_iter = nested.iter();
                let number = solve_node_iter(&mut nested_iter);
                match operation {
                    Some(op) => {
                        match op {
                            MathType::Product => left_number *= number,
                            MathType::Sum => left_number += number,
                            _ => unreachable!(),
                        };
                        operation = None;
                    }
                    None => left_number = number,
                }
            }
        }

        maybe_node_iter = node_iter.next();
    }

    return left_number;
}

pub fn solve_node(node: ParserNode) -> i64 {
    match node {
        ParserNode::Entry(entry) => match entry {
            MathType::Number(number) => {
                return number;
            }
            _ => unreachable!("Invalid entry node."),
        },
        ParserNode::Nested(nested) => {
            let mut nested_iter = nested.iter();
            return solve_node_iter(&mut nested_iter);
        }
    }
}

pub fn solve(equation: &str) -> i64 {
    let result = lexical_analyzer_run(equation);
    let result = parser_run(&result);
    return solve_node(result);
}

fn main() {
    let content = helper::get_input_file("18-input.txt");
    let answers = content
        .lines()
        .map(|line| solve(line))
        .collect::<Vec<i64>>();
    helper::print_answer("18-1", answers.iter().sum::<i64>() as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("1 + 1 * 3"), 6);
        assert_eq!(solve("3 * 1 + 1"), 4);
        assert_eq!(solve("3 * 1 + 1 + (1)"), 5);
        assert_eq!(solve("3 * 1 + 1 + (1 + 1)"), 6);
        assert_eq!(solve("3 * 1 + 1 + (1 + 1 + 1)"), 7);
        assert_eq!(solve("3 * 1 + 1 + (1 + 1 + (1 + 1))"), 8);

        assert_eq!(solve("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(solve("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve("2 * 3 + (4 * 5)"), 26);
        assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
