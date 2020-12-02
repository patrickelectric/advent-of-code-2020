use aoc2020::helper;

use regex::Regex;

pub struct Passwords {
    pub first_number: usize,
    pub second_number: usize,
    pub letter: char,
    pub password: String,
}

pub fn get_password_from_database(filename: &str) -> Vec<Passwords> {
    let values = helper::get_input_file(filename);
    let re = Regex::new(
        r"(?P<first_number>\d+)-(?P<second_number>\d+) (?P<letter>\S): (?P<password>\S+)",
    )
    .unwrap();

    let mut passwords = vec![];
    for value in values.lines() {
        let matches = re.captures(value).unwrap();

        passwords.push(Passwords {
            first_number: matches
                .name("first_number")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            second_number: matches
                .name("second_number")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            letter: matches
                .name("letter")
                .unwrap()
                .as_str()
                .chars()
                .nth(0)
                .unwrap(),
            password: matches.name("password").unwrap().as_str().into(),
        });
    }

    return passwords;
}
