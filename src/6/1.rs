use aoc2020::helper;

fn main() {
    let forms_table = helper::get_input_file("6-input.txt");
    let mut answer: String = forms_table.lines().next().unwrap().to_string();
    let mut answers = vec![];
    for line in forms_table.lines().skip(1) {
        if line.is_empty() {
            if !answer.is_empty() {
                answers.push(answer);
                answer = Default::default();
            }
            continue;
        }
        answer += line;
    }
    answers.push(answer);

    let sort_string = |string: &String| -> u64 {
        let mut chars = string.chars().collect::<Vec<char>>();
        chars.sort();
        chars.dedup();
        return chars.len() as u64;
    };
    let total: u64 = answers
        .iter()
        .map(sort_string)
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    helper::print_answer("6-1", total); // 6789 high
}
