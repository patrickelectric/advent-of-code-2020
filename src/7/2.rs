use aoc2020::helper;

mod bags;

fn main() {
    let rules = helper::get_input_file("7-input.txt");
    let map = bags::get_map_of_bags(&rules);
    let my_bag = "shiny gold";
    let quantity = bags::get_nested_bags_quantity(
        &vec![bags::Bag {
            color: my_bag.into(),
            quantity: 1,
        }],
        &map,
    );
    helper::print_answer("7-2", quantity - 1); // Remove shiny gold
}
