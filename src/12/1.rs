use aoc2020::helper;

use regex::Regex;

#[derive(Debug)]
struct Ferry {
    orientation: i64,
    n: i64,
    e: i64,
    re: Regex,
}

impl Ferry {
    pub fn new() -> Self {
        return Self {
            orientation: 90,
            n: 0,
            e: 0,
            re: Regex::new(&format!(r"(?P<name>\S)(?P<value>\d+)")).unwrap(),
        };
    }

    pub fn update(&mut self, command: &str) {
        let captured = self.re.captures(command).unwrap();
        let value = captured
            .name("value")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        match captured.name("name").unwrap().as_str() {
            "N" => self.n += value,
            "S" => self.n -= value,
            "E" => self.e += value,
            "W" => self.e -= value,
            "L" => self.orientation -= value,
            "R" => self.orientation += value,
            "F" => {
                while self.orientation < 0 {
                    self.orientation += 360;
                }
                while self.orientation >= 360 {
                    self.orientation -= 360;
                }
                match self.orientation {
                    0 => self.n += value,
                    90 => self.e += value,
                    180 => self.n -= value,
                    270 => self.e -= value,
                    _ => unreachable!("wth"),
                }
            }
            _ => unreachable!("wth"),
        }
    }

    pub fn distance(&self) -> i64 {
        return self.n.abs() + self.e.abs();
    }
}

fn main() {
    let commands = helper::get_input_file("12-input.txt");
    let mut ferry = Ferry::new();
    for command in commands.lines() {
        ferry.update(command);
    }
    helper::print_answer("12-1", ferry.distance() as u64);
}
