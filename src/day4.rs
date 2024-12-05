use std::iter::zip;

use crate::utils::read_lines;

pub fn solution1() {
    let puzzle = read_puzzle();

    let horizontal_sum = puzzle
        .iter()
        .map(|line| {
            line.windows(4)
                .filter(|window| {
                    matches!(window, [b'X', b'M', b'A', b'S'] | [b'S', b'A', b'M', b'X'])
                })
                .count() as u32
        })
        .sum::<u32>();
    let vertical_sum = puzzle
        .windows(4)
        .map(|window| {
            zip(
                window[0].iter(),
                zip(window[1].iter(), zip(window[2].iter(), window[3].iter())),
            )
            .map(|(a, (b, (c, d)))| (a, b, c, d))
            .filter(|tup| matches!(tup, (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')))
            .count() as u32
        })
        .sum::<u32>();
    let diag_sum_1 = puzzle
        .windows(4)
        .map(|window| {
            zip(
                window[0].iter().skip(3),
                zip(
                    window[1].iter().skip(2),
                    zip(window[2].iter().skip(1), window[3].iter()),
                ),
            )
            .map(|(a, (b, (c, d)))| (a, b, c, d))
            .filter(|tup| matches!(tup, (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')))
            .count() as u32
        })
        .sum::<u32>();
    let diag_sum_2 = puzzle
        .windows(4)
        .map(|window| {
            zip(
                window[0].iter(),
                zip(
                    window[1].iter().skip(1),
                    zip(window[2].iter().skip(2), window[3].iter().skip(3)),
                ),
            )
            .map(|(a, (b, (c, d)))| (a, b, c, d))
            .filter(|tup| matches!(tup, (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')))
            .count() as u32
        })
        .sum::<u32>();

    println!(
        "XMAS count = {}",
        horizontal_sum + vertical_sum + diag_sum_1 + diag_sum_2
    );
}

pub fn solution2() {
    let puzzle = read_puzzle();

    let sum = puzzle
        .windows(3)
        .map(|window| {
            zip(
                window[0].windows(3),
                zip(window[1].windows(3), window[2].windows(3)),
            )
            .map(|(a, (b, c))| (a, b, c))
            .filter(|tuple| {
                matches!(
                    tuple,
                    ([b'M', _, b'M'], [_, b'A', _], [b'S', _, b'S'])
                        | ([b'S', _, b'M'], [_, b'A', _], [b'S', _, b'M'])
                        | ([b'M', _, b'S'], [_, b'A', _], [b'M', _, b'S'])
                        | ([b'S', _, b'S'], [_, b'A', _], [b'M', _, b'M'])
                )
            })
            .count() as u32
        })
        .sum::<u32>();

    println!("X-MAS count = {sum}");
}

fn read_puzzle() -> Vec<Vec<u8>> {
    read_lines("src/day4/input.txt")
        .map(|line| line.into_bytes())
        .collect()
}
