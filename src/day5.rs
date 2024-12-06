use std::cmp::Ordering;

use crate::utils::read_lines;

pub fn solution1() {
    let mut lines = read_input();

    let rules =
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .fold(Vec::new(), |mut rules, line| {
                let (a, b) = line.as_bytes().split_at(2);
                let a = bytes_to_num(a);
                let b = bytes_to_num(&b[1..]);

                rules.push((a, b));

                rules
            });

    let middle_rules_sum = lines
        .filter_map(|line| {
            let line_nums = line
                .split(',')
                .map(|s| bytes_to_num(s.as_bytes()))
                .collect::<Vec<_>>();
            line_nums
                .is_sorted_by(|&a, &b| rules.iter().any(|&r| r == (a, b)))
                .then_some(line_nums[line_nums.len() / 2])
        })
        .sum::<usize>();

    println!("Sum of in-order middle rules = {middle_rules_sum}");
}

pub fn solution2() {
    let mut lines = read_input();

    let rules =
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .fold(Vec::new(), |mut rules, line| {
                let (a, b) = line.as_bytes().split_at(2);
                let a = bytes_to_num(a);
                let b = bytes_to_num(&b[1..]);

                rules.push((a, b));

                rules
            });

    let middle_rules_sum = lines
        .filter_map(|line| {
            let mut line_nums = line
                .split(',')
                .map(|s| bytes_to_num(s.as_bytes()))
                .collect::<Vec<_>>();
            (!line_nums.is_sorted_by(|&a, &b| rules.iter().any(|&r| r == (a, b)))).then(|| {
                line_nums.sort_by(|&a, &b| {
                    rules
                        .iter()
                        .any(|&r| r == (a, b))
                        .then_some(Ordering::Less)
                        .unwrap_or(Ordering::Greater)
                });

                line_nums[line_nums.len() / 2]
            })
        })
        .sum::<usize>();

    println!("Sum of middle rules = {middle_rules_sum}");
}

fn read_input() -> impl Iterator<Item = String> {
    read_lines("src/day5/input.txt")
}

fn bytes_to_num(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| (*digit - b'0') as usize * 10usize.pow(i as u32))
        .sum::<usize>()
}
