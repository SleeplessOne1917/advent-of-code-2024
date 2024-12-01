use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Must specify file to open");
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line!"))
}
