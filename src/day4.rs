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
    let vertical_and_diagonal_sum = puzzle
        .windows(4)
        .map(|window| {
            count_xmas(window, (0, 0, 0, 0))
                + count_xmas(window, (0, 1, 2, 3))
                + count_xmas(window, (3, 2, 1, 0))
        })
        .sum::<u32>();

    println!(
        "XMAS count = {}",
        horizontal_sum + vertical_and_diagonal_sum
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

fn count_xmas(
    window: &[Vec<u8>],
    (skip0, skip1, skip2, skip3): (usize, usize, usize, usize),
) -> u32 {
    zip(
        window[0].iter().skip(skip0),
        zip(
            window[1].iter().skip(skip1),
            zip(window[2].iter().skip(skip2), window[3].iter().skip(skip3)),
        ),
    )
    .map(|(a, (b, (c, d)))| (a, b, c, d))
    .filter(|tup| matches!(tup, (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')))
    .count() as u32
}

fn read_puzzle() -> Vec<Vec<u8>> {
    read_lines("src/day4/input.txt")
        .map(|line| line.into_bytes())
        .collect()
}
