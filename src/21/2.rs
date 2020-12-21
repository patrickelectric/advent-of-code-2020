use aoc2020::helper;

mod foods;
use foods::Foods;

fn main() {
    let content = helper::get_input_file("21-input.txt");
    let mut foods = Foods::new();
    content.lines().for_each(|line| foods.process(line));
    foods.pos_processing();
    println!("21-2 answer: {}", foods.dangerous_list());
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
        assert_eq!(foods.dangerous_list(), "mxmxvkd,sqjhc,fvjkl");
    }
}
