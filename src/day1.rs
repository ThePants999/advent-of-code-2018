use std::collections::HashSet;

pub fn day1(input_lines: &[String]) -> (String, String) {
    let changes: Vec<i64> = input_lines.iter().map(|line| line.parse::<i64>().expect("Failed to parse input")).collect();
    let part1 = changes.iter().sum::<i64>().to_string();
    let mut seen_freqs: HashSet<i64> = HashSet::new();
    let mut freq = 0i64;
    let part2 = 'outer: loop {
        for change in &changes {
            freq += change;
            if seen_freqs.contains(&freq) {
                break 'outer freq;
            } else {
                seen_freqs.insert(freq);
            }
        }
    }.to_string();
    (part1, part2) 
}