use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .collect()
}


fn main() {
    let file = File::open("./input/01.txt").expect("Failed to open file");
    let lines = read_lines(file).expect("Incorrect input");

    for line in lines {
        println!("{}", line);
    }
}
