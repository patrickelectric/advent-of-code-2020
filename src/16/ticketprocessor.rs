use aoc2020::helper;

use regex::Regex;

use std::collections::HashMap;

#[derive(Debug)]
pub struct TicketCategory {
    name: String,
    range: Vec<(u64, u64)>,
}

impl TicketCategory {
    pub fn is_valid(&self, value: u64) -> bool {
        return self
            .range
            .iter()
            .filter(|range| range.0 <= value && value <= range.1)
            .count()
            != 0;
    }
}

#[derive(Debug)]
pub struct TicketProcessor {
    categories: Vec<TicketCategory>,
    pub tickets: Vec<Vec<u64>>,
}

impl TicketProcessor {
    pub fn new() -> Self {
        return Self {
            categories: vec![],
            tickets: vec![],
        };
    }

    pub fn load(&mut self, filename: &str) {
        let content = helper::get_input_file(filename);

        let category_regex = Regex::new(r"(?P<category>.*): (?P<range>\d+-\d+)").unwrap();
        let range_regex = Regex::new(r"(?P<range>(?P<lower>\d+)-(?P<higher>\d+))").unwrap();

        let mut is_category_group = true;

        for line in content.lines() {
            if is_category_group {
                match category_regex.captures(line) {
                    Some(category_content) => {
                        let name = category_content.name("category").unwrap().as_str().into();
                        let range: Vec<(u64, u64)> = range_regex
                            .captures_iter(line)
                            .map(|capture| {
                                (
                                    capture
                                        .name("lower")
                                        .unwrap()
                                        .as_str()
                                        .parse::<u64>()
                                        .unwrap(),
                                    capture
                                        .name("higher")
                                        .unwrap()
                                        .as_str()
                                        .parse::<u64>()
                                        .unwrap(),
                                )
                            })
                            .collect();
                        self.categories.push(TicketCategory { name, range });
                        continue;
                    }
                    _ => is_category_group = false,
                }
            }

            if line.chars().filter(|&x| x == ',').count() == 0 {
                continue;
            }

            self.tickets.push(
                line.split(',')
                    .map(|value| value.parse::<u64>().unwrap())
                    .collect(),
            );
        }
    }

    fn is_valid_ticket(&self, ticket: &Vec<u64>) -> bool {
        let ticket = ticket
            .iter()
            .filter(|&&value| {
                self.categories
                    .iter()
                    .filter(|category| category.is_valid(value))
                    .count()
                    != 0
            })
            .collect::<Vec<&u64>>();
        return ticket.len() == self.categories.len();
    }

    pub fn error_rate(&self) -> u64 {
        let invalid_values = self
            .tickets
            .iter()
            .map(|ticket| {
                ticket
                    .iter()
                    .filter(|&&value| {
                        self.categories
                            .iter()
                            .filter(|category| category.is_valid(value))
                            .count()
                            == 0
                    })
                    .collect::<Vec<&u64>>()
            })
            .collect::<Vec<Vec<&u64>>>();
        let sum: u64 = invalid_values
            .iter()
            .map(|vector| vector.iter().map(|val| *val).sum::<u64>())
            .sum();
        return sum;
    }

    pub fn remove_invalid_tickets(&mut self) {
        self.tickets = self
            .tickets
            .iter()
            .filter(|&ticket| self.is_valid_ticket(ticket))
            .cloned()
            .collect();
    }

    pub fn multiply_departures(&mut self) -> u64 {
        let magic_prefix = "departure";
        let mut category_per_index: HashMap<String, Vec<u64>> = self
            .categories
            .iter()
            .map(|category| {
                (
                    category.name.clone(),
                    (0..self.categories.len() as u64).collect::<Vec<u64>>(),
                )
            })
            .collect();

        // Filter categories with valid index
        for ticket in self.tickets.iter() {
            for (index, &value) in ticket.iter().enumerate() {
                let index = index as u64;
                for category in self.categories.iter() {
                    if category_per_index[&category.name].contains(&index) {
                        if !category.is_valid(value) {
                            category_per_index
                                .get_mut(&category.name)
                                .unwrap()
                                .retain(|&value| value != index);
                        }
                    }
                }
            }
        }

        // Remove common index between categories
        let mut single_index = vec![];
        while category_per_index
            .values()
            .any(|category| category.len() != 1)
        {
            for values in category_per_index.values_mut() {
                if values.len() == 1 {
                    let &value = values.first().unwrap();
                    if !single_index.contains(&value) {
                        single_index.push(value);
                    }
                    continue;
                }

                values.retain(|value| !single_index.contains(value));
            }
        }

        let my_ticket = self.tickets.first().unwrap();
        let valid_indexes: Vec<u64> = category_per_index
            .iter()
            .filter(|(name, _)| name.contains(magic_prefix))
            .map(|(_, values)| *values.first().unwrap())
            .collect();
        let result: u64 = my_ticket
            .iter()
            .enumerate()
            .filter(|&(index, _)| valid_indexes.contains(&(index as u64)))
            .map(|(_, value)| value)
            .product();
        return result;
    }
}
