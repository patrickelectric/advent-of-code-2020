use aoc2020::helper;

use regex::Regex;

#[derive(Default, Debug, Clone)]
pub struct Passport {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl Passport {
    //TODO: Move to macro
    pub fn update_from_string(&mut self, line: &str) {
        let re = Regex::new(&format!(r"(?P<name>\S+):(?P<value>\S+)\b")).unwrap();
        let values = line.split(" ");

        for value in values {
            let captured = re.captures(value).unwrap();

            match captured.name("name").unwrap().as_str() {
                "byr" => {
                    self.byr = Some(captured.name("value").unwrap().as_str().into());
                }
                "iyr" => {
                    self.iyr = Some(captured.name("value").unwrap().as_str().into());
                }
                "eyr" => {
                    self.eyr = Some(captured.name("value").unwrap().as_str().into());
                }
                "hgt" => {
                    self.hgt = Some(captured.name("value").unwrap().as_str().into());
                }
                "hcl" => {
                    self.hcl = Some(captured.name("value").unwrap().as_str().into());
                }
                "ecl" => {
                    self.ecl = Some(captured.name("value").unwrap().as_str().into());
                }
                "pid" => {
                    self.pid = Some(captured.name("value").unwrap().as_str().into());
                }
                "cid" => {
                    self.cid = Some(captured.name("value").unwrap().as_str().into());
                }
                _ => {
                    println!("what: {:#?}", captured);
                }
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }

    pub fn has_valid_data(&self) -> bool {
        let mut valid = self.is_valid();
        if !valid {
            return false;
        }

        let element = self.byr.as_ref();
        if let Ok(year) = element.unwrap().parse::<u32>() {
            valid &= year >= 1920 && year <= 2002;
        }

        let element = self.iyr.as_ref();
        if let Ok(year) = element.unwrap().parse::<u32>() {
            valid &= year >= 2010 && year <= 2020;
        }

        let element = self.eyr.as_ref();
        if let Ok(year) = element.unwrap().parse::<u32>() {
            valid &= year >= 2020 && year <= 2030;
        }

        let element = self.hgt.as_ref();
        if let Some(height) = element {
            let re = Regex::new(r"\b(?P<value>\d+)(?P<type>cm|in)\b").unwrap();
            let captured = re.captures(&height);
            if captured.is_none() {
                return false;
            }

            let captured = captured.unwrap();

            let height = captured.name("value");
            let height_type = captured.name("type");

            if height.is_none() || height_type.is_none() {
                return false;
            }
            let height = height.unwrap().as_str();
            let height_type = height_type.unwrap().as_str();

            let mut result = false;

            if "cm" == height_type {
                if let Ok(height) = height.parse::<u32>() {
                    result = height >= 150 && height <= 193;
                }
            } else if "in" == height_type {
                if let Ok(height) = height.parse::<u32>() {
                    result = height >= 59 && height <= 76;
                }
            }

            valid &= result;
        }

        let element = self.hcl.as_ref();
        if let Some(color) = element {
            let re = Regex::new(r"#[\d|a-f]{6}\b").unwrap();
            valid &= re.is_match(&color);
        }

        let element = self.ecl.as_ref();
        if let Some(color) = element {
            valid &= ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color.as_str());
        }

        let element = self.pid.as_ref();
        if let Some(color) = element {
            let re = Regex::new(r"\b[\d]{9}\b").unwrap();
            valid &= re.is_match(&color);
        }

        return valid;
    }
}

pub fn scan_batch_file(filename: &str) -> Vec<Passport> {
    let content = helper::get_input_file(filename);

    let mut passports: Vec<Passport> = vec![];

    let mut passport: Passport = Default::default();

    for line in content.lines() {
        if line.is_empty() {
            passports.push(passport);
            passport = Default::default();
            continue;
        }
        passport.update_from_string(line);
    }
    passports.push(passport);

    return passports;
}
