use crate::utils::read_lines;

pub fn solution1() {
    let reports = get_reports();
    let safe_reports = reports
        .filter(|report| report.windows(3).all(window_is_valid))
        .count();

    println!("Number of safe reports = {safe_reports}");
}

pub fn solution2() {
    let reports = get_reports();
    let safe_reports = reports
        .filter(|report| {
            report.windows(3).all(window_is_valid)
                || (0..report.len()).any(|i| {
                    [&report[0..i], &report[i + 1..]]
                        .concat()
                        .windows(3)
                        .all(window_is_valid)
                })
        })
        .count();

    println!("Number of safe reports = {safe_reports}");
}

fn window_is_valid(window: &[usize]) -> bool {
    matches!(window[0].abs_diff(window[1]), 1..=3)
        && matches!(window[1].abs_diff(window[2]), 1..=3)
        && ((window[0] > window[1] && window[1] > window[2])
            || (window[0] < window[1] && window[1] < window[2]))
}

fn get_reports() -> impl Iterator<Item = Vec<usize>> {
    read_lines("src/day2/input.txt").map(|line| {
        line.split_ascii_whitespace()
            .map(|level| {
                level
                    .parse()
                    .expect("Reactor level is always valid integer")
            })
            .collect()
    })
}
