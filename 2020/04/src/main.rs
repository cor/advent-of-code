extern crate regex;
use regex::Regex;
use std::fs::File;
use std::str::FromStr;
use std::io::Read;

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
            Self::BirthYear(n) => (1920..2003).contains(n),
            Self::IssueYear(n) => (2010..2021).contains(n),
            Self::ExpirationYear(n) => (2020..2031).contains(n),
            Self::HairColor(s) => {
                let input_re: Regex = Regex::new(r#"#([a-f0-9]{6})"#).unwrap();
                input_re.captures_iter(s).count() > 0
            },
            Self::EyeColor(c) => {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&c.as_str())
            },
            Self::PassportID(s) => {
                let input_re: Regex = Regex::new(r#"^(\d{9})$"#).unwrap();
                input_re.captures_iter(s).count() > 0
            },
            Self::CountryID(_) => true,
            Self::Height(s) => {
                let input_re: Regex = Regex::new(
                    r#"(?x)
                           (\d+)(cm) |
                           (\d+)(in)
                           "#).unwrap();

                let captures = input_re.captures(s).map(|captures| {
                    captures
                        .iter() // All the captured groups
                        .skip(1) // Skipping the complete match
                        .flat_map(|c| c) // Ignoring all empty optional matches
                        .map(|c| c.as_str()) // Grab the original strings
                        .collect::<Vec<_>>() // Create a vector
                });


                // Match against the captured values as a slice
                match captures.as_ref().map(|c| c.as_slice()) {
                    Some([n, "cm"]) => {
                        let h = n.parse::<u64>().unwrap();
                        (150..195).contains(&h)
                    },
                    Some([n, "in"]) => {
                        let h = n.parse::<u64>().unwrap();
                        (59..77).contains(&h)
                    },
                    _ => false,
                }
            },
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
    fn is_valid (&self) -> bool {
        let all_8 = self.0.len() == 8;
        let without_country = self.0.len() == 7 &&
            !self.0.iter().any(|f| matches!(f, Field::CountryID(_)));

        all_8 || without_country
    }

    fn is_valid_2 (&self) -> bool {
        self.is_valid() && self.0.iter().all(|f| f.is_valid())
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
        self.0.iter()
            .filter(|p| Passport::is_valid(p))
            .count()
    }

    fn valid_count_2(&self) -> usize {
        self.0.iter()
            .filter(|p| Passport::is_valid_2(p))
            .count()
    }
}

impl FromStr for Passports {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .lines()
            .fold(Vec::from([String::new()]), |mut acc: Vec<String>, l: &str| {
                match l {
                    "" => acc.push(String::new()), // Start a new Passport string for each newline
                    _ => acc.last_mut()
                        .unwrap()
                        .push_str(format!(" {}", l).as_str()), // Add fields to last passport
                }
                acc
            })
            .iter()
            .map(|s| Passport::from_str(s))
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
