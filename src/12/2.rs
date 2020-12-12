use aoc2020::helper;

use regex::Regex;

#[derive(Debug)]
struct Waypoint {
    n: i64,
    e: i64,
}

#[derive(Debug)]
struct Ferry {
    n: i64,
    e: i64,
    re: Regex,
    waypoint: Waypoint,
}

impl Ferry {
    pub fn new() -> Self {
        return Self {
            n: 0,
            e: 0,
            re: Regex::new(&format!(r"(?P<name>\S)(?P<value>\d+)")).unwrap(),
            waypoint: Waypoint { n: 1, e: 10 },
        };
    }

    pub fn rotate(&mut self, angle: i64) {
        let angle = (angle as f64) * std::f64::consts::PI / 180.0;
        let n = ((self.waypoint.n as f64) * angle.cos()) as i64
            - ((self.waypoint.e as f64) * angle.sin()) as i64;
        let e = ((self.waypoint.n as f64) * angle.sin()) as i64
            + ((self.waypoint.e as f64) * angle.cos()) as i64;
        self.waypoint.n = n;
        self.waypoint.e = e;
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
            "N" => self.waypoint.n += value,
            "S" => self.waypoint.n -= value,
            "E" => self.waypoint.e += value,
            "W" => self.waypoint.e -= value,
            "L" => self.rotate(-value),
            "R" => self.rotate(value),
            "F" => {
                self.n += value * self.waypoint.n;
                self.e += value * self.waypoint.e;
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
    helper::print_answer("12-2", ferry.distance() as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut ferry = Ferry::new();

        ferry.update("F10");
        assert_eq!(ferry.n, 10);
        assert_eq!(ferry.e, 100);
        assert_eq!(ferry.waypoint.n, 1);
        assert_eq!(ferry.waypoint.e, 10);

        ferry.update("N3");
        assert_eq!(ferry.n, 10);
        assert_eq!(ferry.e, 100);
        assert_eq!(ferry.waypoint.n, 4);
        assert_eq!(ferry.waypoint.e, 10);

        ferry.update("F7");
        assert_eq!(ferry.n, 38);
        assert_eq!(ferry.e, 170);
        assert_eq!(ferry.waypoint.n, 4);
        assert_eq!(ferry.waypoint.e, 10);

        ferry.update("R90");
        assert_eq!(ferry.n, 38);
        assert_eq!(ferry.e, 170);
        assert_eq!(ferry.waypoint.n, -10);
        assert_eq!(ferry.waypoint.e, 4);

        ferry.update("F11");
        assert_eq!(ferry.n, -72);
        assert_eq!(ferry.e, 214);
        assert_eq!(ferry.waypoint.n, -10);
        assert_eq!(ferry.waypoint.e, 4);
    }
}
