use crate::utils::read_lines;

pub fn solution1() {
    let reports = read_lines("src/day2/input.txt").map(|line| {
        line.split_ascii_whitespace()
            .map(|level| {
                level
                    .parse()
                    .expect("Reactor level is always valid integer")
            })
            .collect::<Vec<usize>>()
    });

    let safe_reports = reports
        .filter(|report| {
            report.windows(3).all(|window| {
                matches!(window[0].abs_diff(window[1]), 1..=3)
                    && matches!(window[1].abs_diff(window[2]), 1..=3)
                    && ((window[0] > window[1] && window[1] > window[2])
                        || (window[0] < window[1] && window[1] < window[2]))
            })
        })
        .count();

    println!("Number of safe reports = {safe_reports}");
}
