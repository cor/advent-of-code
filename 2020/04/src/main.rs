use std::fs::File;
use std::str::FromStr;
use std::string::ParseError;
use std::io::Read;


type Passport = Vec<Field>;

#[derive(Debug)]
enum Field {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(usize),
    CountryID(usize),
}

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split(":").collect();

        // TODO: Finish
        Ok(Field::Height(key.append(value)))
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

    println!("Hello, world!");
}
