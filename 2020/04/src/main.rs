extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

#[derive(Debug, PartialEq)]
enum Field {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(u32),
}

impl Field {
    fn is_valid(&self) -> bool {
        match self {
            Self::BirthYear(n) => (1920..=2002).contains(n),
            Self::IssueYear(n) => (2010..=2020).contains(n),
            Self::ExpirationYear(n) => (2020..=2030).contains(n),
            Self::HairColor(s) => {
                let input_re: Regex = Regex::new(r#"#([a-f0-9]{6})"#).unwrap();
                input_re.captures_iter(s).count() > 0
            }
            Self::EyeColor(c) => {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&c.as_str())
            }
            Self::PassportID(s) => {
                let input_re: Regex = Regex::new(r#"^(\d{9})$"#).unwrap();
                input_re.captures_iter(s).count() > 0
            }
            Self::CountryID(_) => true,
            Self::Height(s) => {
                let num = (&s[..(s.len() - 2)]).parse::<u64>();

                if let Ok(h) = num {
                    if s.ends_with("cm") {
                        return (150..=194).contains(&h);
                    } else if s.ends_with("in") {
                        return (59..=76).contains(&h);
                    }
                }
                false
            }
        }
    }
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(":").collect();
        let key = items[0];
        let value = items[1];

        match key {
            "byr" => Ok(Self::BirthYear(value.parse::<u32>().unwrap())),
            "iyr" => Ok(Self::IssueYear(value.parse::<u32>().unwrap())),
            "eyr" => Ok(Self::ExpirationYear(value.parse::<u32>().unwrap())),
            "hgt" => Ok(Self::Height(String::from(value))),
            "hcl" => Ok(Self::HairColor(String::from(value))),
            "ecl" => Ok(Self::EyeColor(String::from(value))),
            "pid" => Ok(Self::PassportID(String::from(value))),
            "cid" => Ok(Self::CountryID(value.parse::<u32>().unwrap())),
            _ => Err(String::from("Invalid Field key")),
        }
    }
}

#[derive(Debug)]
struct Passport(Vec<Field>);

impl Passport {
    fn is_valid(&self) -> bool {
        let no_country_id = !self.0.iter().any(|f| matches!(f, Field::CountryID(_)));
        self.0.len() == 8 || (self.0.len() == 7 && no_country_id)
    }

    fn is_valid_2(&self) -> bool {
        self.is_valid() && self.0.iter().all(Field::is_valid)
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| Field::from_str(f))
            .filter_map(Result::ok)
            .collect();

        Ok(Passport(fields))
    }
}

#[derive(Debug)]
struct Passports(Vec<Passport>);

impl Passports {
    fn valid_count_1(&self) -> usize {
        self.0.iter().filter(|p| Passport::is_valid(p)).count()
    }

    fn valid_count_2(&self) -> usize {
        self.0.iter().filter(|p| Passport::is_valid_2(p)).count()
    }
}

impl FromStr for Passports {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .trim()
            .split("\n\n")
            .map(|x| x.to_string().replace("\n", " "))
            .map(|s| Passport::from_str(s.as_str()))
            .filter_map(Result::ok)
            .collect();

        Ok(Passports(res))
    }
}

fn main() {
    let input = load_file("./input/1.txt");

    let passports = Passports::from_str(&input).expect("Couldn't parse passports");

    println!("{}", passports.valid_count_1());
    println!("{}", passports.valid_count_2());
}
