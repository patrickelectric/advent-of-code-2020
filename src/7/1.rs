use aoc2020::helper;

mod bags;

fn main() {
    let rules = helper::get_input_file("7-input.txt");
    let map = bags::get_map_of_bags(&rules);

    let my_bag = "shiny gold";
    let mut possible_parent_bags = bags::find_bags_that_contain(my_bag, &map);

    let mut last_possible_parent_bags_length = 0;
    loop {
        let mut new_bags = vec![];
        for bag_color in &possible_parent_bags {
            new_bags.append(&mut bags::find_bags_that_contain(bag_color, &map).clone());
        }

        possible_parent_bags.append(&mut new_bags.clone());
        possible_parent_bags.sort();
        possible_parent_bags.dedup();

        if last_possible_parent_bags_length == possible_parent_bags.len() {
            break;
        }
        last_possible_parent_bags_length = possible_parent_bags.len();
    }

    helper::print_answer("7-1", possible_parent_bags.len() as u64);
}
