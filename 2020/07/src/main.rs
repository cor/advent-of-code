use aoc_2020_common::common::load_file;
use regex::Regex;
use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {

    let contains_re: Regex = Regex::new(r#"(\d+) (.+?) bag"#).unwrap();
    let mut can_be_contained_in: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        if !line.ends_with("no other bags") {
            let split: Vec<&str> = line.split(" bags contain").collect();
            let container_color = split[0].to_owned();
            for cap in contains_re.captures_iter(&split[1]) {
                let contained_color = cap[2].to_owned();
                let entry = can_be_contained_in.entry(contained_color).or_insert(Vec::new());
                (*entry).push(container_color.clone());
            }
        }
    }

    can_be_contained_in
}

fn main() {
    let input = load_file("./input/example.txt");

    let can_be_contained_in = parse_input(&input);

    println!("map '{:#?}'", can_be_contained_in);
}
