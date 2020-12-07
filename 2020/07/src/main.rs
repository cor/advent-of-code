use aoc_2020_common::common::load_file;
use regex::Regex;
use std::collections::HashMap;

fn parse_input(input: &str) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<(usize, String)>>) {
    let contains_re: Regex = Regex::new(r#"(\d+) (.+?) bag"#).unwrap();
    let mut can_be_contained_in: HashMap<String, Vec<String>> = HashMap::new();
    let mut contains: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in input.lines() {
        if !line.ends_with("no other bags") {
            let split: Vec<&str> = line.split(" bags contain").collect();
            let container_color = split[0].to_owned();
            for cap in contains_re.captures_iter(&split[1]) {
                let contained_count = cap[1].parse::<usize>().unwrap();
                let contained_color = cap[2].to_owned();

                // Add to can_be_contained_in map for part 1
                let entry = can_be_contained_in.entry(contained_color.clone()).or_insert(Vec::new());
                (*entry).push(container_color.clone());

                // Add to contains map for part 2
                let entry = contains.entry(container_color.clone()).or_insert(Vec::new());
                (*entry).push((contained_count, contained_color));
            }
        }
    }

    (can_be_contained_in, contains)
}

fn add_to_containers(target: &String, containers: &mut Vec<String>, can_be_contained_in: &HashMap<String, Vec<String>>) {
    if can_be_contained_in.contains_key(target) {
        for container in &can_be_contained_in[target] {
            add_to_containers(container, containers, can_be_contained_in);
            containers.push(container.clone());
        }
    }
}

fn count_bags(target: &String, containers: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut total = 0;
    if containers.contains_key(target) {
        for (count, color) in &containers[target] {
            total += count * (1 + count_bags(color, &containers));
        }
    }

    total
}

fn main() {
    let input = load_file("./input/1.txt");
    let target = String::from("shiny gold");

    // can_be_contained_in is for answer 1, contains is for answer 2
    let (can_be_contained_in, contains) = parse_input(&input);

    // Answer 1
    let mut containers : Vec<String> = Vec::new();
    add_to_containers(&target, &mut containers, &can_be_contained_in);
    containers.sort_unstable();
    containers.dedup();
    let answer1 = containers.len();
    println!("{:#?}", answer1);

    // Answer 2
    let answer2 = count_bags(&target, &contains);
    println!("{:#?}", answer2);
}
