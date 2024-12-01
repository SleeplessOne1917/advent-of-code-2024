use crate::utils::read_lines;

pub fn solution1() {
    let (mut group_1_ids, mut group_2_ids): (Vec<_>, Vec<_>) = read_lines("src/day1/input.txt")
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
        .unzip();

    group_1_ids.sort();
    group_2_ids.sort();

    let total_distance = group_1_ids
        .into_iter()
        .zip(group_2_ids.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("Total distance = {total_distance}");
}
