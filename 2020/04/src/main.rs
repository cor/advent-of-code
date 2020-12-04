use std::fs::File;
use std::str::FromStr;
use std::string::ParseError;
use std::io::Read;


type Passport = Vec<Field>;

#[derive(Debug)]
enum Field {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(u32),
    CountryID(u32),
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
            "pid" => Ok(Self::PassportID(value.parse::<u32>().unwrap())),
            "cid" => Ok(Self::CountryID(value.parse::<u32>().unwrap())),
            _ => Err(String::from("Invalid Field key")),
        }
    }
}


fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

fn main() {
    let input = load_file("./input/1.txt");

    let field = Field::from_str("hcl:#ae17e1");

    println!("{:?}", field);
}
