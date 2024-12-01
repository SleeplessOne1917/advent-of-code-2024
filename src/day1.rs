use std::collections::HashMap;

use crate::utils::read_lines;

pub fn solution1() {
    let (mut id_list1, mut id_list2) = get_id_lists();

    id_list1.sort();
    id_list2.sort();

    let total_distance = id_list1
        .into_iter()
        .zip(id_list2)
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>();

    println!("Total distance = {total_distance}");
}

pub fn solution2() {
    let (id_list1, id_list2) = get_id_lists();

    let id_count_map = id_list2
        .into_iter()
        .fold(HashMap::<_, i32>::new(), |mut map, id| {
            *map.entry(id).or_default() += 1i32;

            map
        });

    let similarity_score = id_list1
        .into_iter()
        .map(|id| id * id_count_map.get(&id).copied().unwrap_or_default())
        .sum::<i32>();

    println!("Similarity score = {similarity_score}");
}

fn get_id_lists() -> (Vec<i32>, Vec<i32>) {
    read_lines("src/day1/input.txt")
        .map(|line| {
            let mut ids = line.split_whitespace().map(|id| {
                id.parse::<i32>()
                    .expect("Ids from input must be valid integers")
            });

            (
                ids.next().expect("First Id on line must be present"),
                ids.next().expect("Second Id on line must be present"),
            )
        })
        .unzip()
}
