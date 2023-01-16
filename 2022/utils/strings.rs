use std::io::{Result, BufReader, BufRead};
use std::fs::{File};
use std::path::Path;

pub fn read_lines<P>(path: P) -> Result<std::io::Lines<BufReader<File>>> 
where P: AsRef<Path>,  {
    let file = File::open(path)
        .expect("Path doens't exists");
    let reader = BufReader::new(file);
   Ok(reader.lines())
}

