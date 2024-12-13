use crate::utils::read_lines;
use std::collections::HashSet;
use Direction::*;

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub fn solution1() {
    let input = get_input();

    let (mut i, mut j) = input
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &c)| (c == b'^').then_some((i, j)))
        })
        .expect("The guard is definitely there.");

    let mut visited_coordinates = HashSet::new();
    visited_coordinates.insert((i, j));

    let mut direction = Up;
    'outer: loop {
        match direction {
            Up => {
                while input[i][j] != b'#' {
                    let Some(new_i) = i.checked_sub(1) else {
                        break 'outer;
                    };

                    i = new_i;
                    visited_coordinates.insert((i, j));
                }

                visited_coordinates.remove(&(i, j));
                i += 1;
                direction = Right;
            }
            Left => {
                while input[i][j] != b'#' {
                    let Some(new_j) = j.checked_sub(1) else {
                        break 'outer;
                    };

                    j = new_j;
                    visited_coordinates.insert((i, j));
                }

                visited_coordinates.remove(&(i, j));
                j += 1;
                direction = Up;
            }
            Right => {
                while input[i][j] != b'#' {
                    j += 1;
                    if j >= input[i].len() {
                        break 'outer;
                    }

                    visited_coordinates.insert((i, j));
                }

                visited_coordinates.remove(&(i, j));
                j -= 1;
                direction = Down;
            }
            Down => {
                while input[i][j] != b'#' {
                    i += 1;
                    if i >= input.len() {
                        break 'outer;
                    }

                    visited_coordinates.insert((i, j));
                }

                visited_coordinates.remove(&(i, j));
                i -= 1;
                direction = Left;
            }
        }
    }

    println!(
        "The guard visited {} distinct positions.",
        visited_coordinates.len()
    );
}

fn get_input() -> Vec<Vec<u8>> {
    read_lines("src/day6/input.txt")
        .map(|line| line.bytes().collect())
        .collect()
}
