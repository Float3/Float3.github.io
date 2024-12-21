pub fn solve(input: &str) -> String {
    let safe_count = input
        .trim()
        .lines()
        .map(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|token| token.parse::<i32>().ok())
                .collect();

            if levels.len() < 2 {
                return false;
            }

            let diffs: Vec<i32> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();

            if diffs[0] == 0 {
                return false;
            }

            let first_sign = diffs[0].signum();

            diffs
                .iter()
                .all(|&d| d.signum() == first_sign && (1..=3).contains(&d.abs()))
        })
        .filter(|&is_safe| is_safe)
        .count();

    safe_count.to_string()
}
