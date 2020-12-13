use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bag {
    pub color: String,
    pub quantity: u64,
}

pub fn get_map_of_bags(rules: &str) -> HashMap<&str, Vec<Bag>> {
    let mut map: HashMap<&str, Vec<Bag>> = Default::default();

    let main_bag_regex = Regex::new(r"(?P<color>.*) bags").unwrap();
    let children_bag_regex = Regex::new(r"(?P<quantity>\d+) (?P<color>\S+ \S+)").unwrap();

    for line in rules.lines() {
        let mut bags = line.split("contain").into_iter();
        let main_bag = bags.next().unwrap().trim();
        let bags = bags.next().unwrap().trim();

        let main_bag = main_bag_regex
            .captures(main_bag)
            .unwrap()
            .name("color")
            .unwrap()
            .as_str();

        let bags: Vec<Bag> = children_bag_regex
            .captures_iter(bags)
            .map(|capture| Bag {
                color: capture.name("color").unwrap().as_str().into(),
                quantity: capture.name("quantity").unwrap().as_str().parse().unwrap(),
            })
            .collect();

        map.insert(main_bag, bags);
    }

    return map;
}

pub fn find_bags_that_contain(color: &str, map: &HashMap<&str, Vec<Bag>>) -> Vec<String> {
    let mut colors = vec![];
    for (&main_color, bags) in map {
        for bag in bags {
            if bag.color == color {
                colors.push(main_color.clone().into());
            }
        }
    }
    return colors;
}

pub fn get_bags_inside_bag(color: &str, map: &HashMap<&str, Vec<Bag>>) -> Vec<Bag> {
    for (&bag, bags) in map {
        if bag == color {
            return bags.clone();
        }
    }
    return vec![];
}

pub fn get_nested_bags_quantity(bags: &[Bag], map: &HashMap<&str, Vec<Bag>>) -> u64 {
    let mut total = 0;
    for bag in bags {
        let inside_bags = get_bags_inside_bag(&bag.color, map);
        let local_quantity =
            bag.quantity + bag.quantity * get_nested_bags_quantity(&inside_bags, map);
        total += local_quantity;
    }
    return total;
}
