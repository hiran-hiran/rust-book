use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let dicfile = "./src/ejdict-hand-utf8.txt";

    let args: Vec<String> = args().collect();

    let word = &args[1];

    let file = File::open(dicfile).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(word) {
            if line.find(word).is_none() {
                continue;
            }
            println!("{}", line);
        }
    }
}
