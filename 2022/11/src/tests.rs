use super::*;

#[test]
fn can_parse_monkey() {
    let result = Monkey::parse("Monkey 0:\n  Starting items: 59, 65, 86, 56, 74, 57, 56\n  Operation: new = old * 17\n  Test: divisible by 3\n    If true: throw to monkey 3\n    If false: throw to monkey 6");
    assert_eq!(
        result,
        Ok((
            "",
            Monkey {
                items: vec![59, 65, 86, 56, 74, 57, 56],
                operation: Op::Times(17),
                test: 3,
                targets: (3, 6)
            }
        ))
    );
}

#[test]
fn can_parse_starting_items() {
    assert_eq!(
        parse_starting_items("  Starting items: 54, 65, 75, 74\n")
            .unwrap()
            .1,
        vec![54, 65, 75, 74]
    );
}

#[test]
fn can_parse_op_times() {
    assert_eq!(
        Op::parse("  Operation: new = old * 19\n").unwrap().1,
        Op::Times(19)
    );
}

#[test]
fn can_parse_op_plus() {
    assert_eq!(
        Op::parse("  Operation: new = old + 24\n").unwrap().1,
        Op::Add(24)
    );
}

#[test]
fn can_parse_op_square() {
    assert_eq!(
        Op::parse("  Operation: new = old * old\n").unwrap().1,
        Op::Square
    );
}

#[test]
fn can_parse_targets() {
    assert_eq!(
        parse_targets("    If true: throw to monkey 0\n    If false: throw to monkey 1")
            .unwrap()
            .1,
        (0, 1)
    );
}

#[test]
fn can_parse_test() {
    assert_eq!(parse_test("  Test: divisible by 19\n").unwrap().1, 19);
}
