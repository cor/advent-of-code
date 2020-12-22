#[macro_use] extern crate lalrpop_util;
use aoc_2020_common::common::load_file;

lalrpop_mod!(pub calculator);



fn main() {
    let input = load_file("./input/1.txt");
    println!("{}", input);
}


#[test]
fn calculator() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}