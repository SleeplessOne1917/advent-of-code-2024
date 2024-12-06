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

pub fn bytes_to_num(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| (*digit - b'0') as usize * 10usize.pow(i as u32))
        .sum::<usize>()
}
