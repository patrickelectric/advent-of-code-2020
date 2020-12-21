use aoc2020::helper;

mod foods;
use foods::Foods;

fn main() {
    let content = helper::get_input_file("21-input.txt");
    let mut foods = Foods::new();
    content.lines().for_each(|line| foods.process(line));
    foods.pos_processing();
    helper::print_answer("21-1", foods.unknown_ingredients().len() as u64); // low: 192
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

        let mut foods = Foods::new();
        for line in content.lines() {
            foods.process(line);
        }

        foods.pos_processing();

        assert_eq!(foods.unknown_ingredients().len(), 5);
    }
}
