pub fn day2(input_lines: &[String]) -> (String, String) {
    let mut num_contains_two = 0u64;
    let mut num_contains_three = 0u64;
    let mut part2: Option<String> = None;
    for (index, line) in input_lines.iter().enumerate() {
        let (contains_two, contains_three) = evaluate_id(line);
        if contains_two { num_contains_two += 1; }
        if contains_three { num_contains_three += 1; }

        if part2.is_none() {
            for other_line in input_lines.iter().skip(index + 1) {
                if let Some(result) = compare_ids(line, other_line) {
                    part2 = Some(result);
                    break;
                }
            }
        }
    }

    let part1 = (num_contains_two * num_contains_three).to_string();
    (part1, part2.unwrap())
}

fn evaluate_id(id: &str) -> (bool, bool) {
    let mut counts: [u8; 26] = [0; 26];
    for c in id.chars() {
        counts[c as usize - 'a' as usize] += 1;
    }
    let contains_two = counts.iter().any(|&count| count == 2);
    let contains_three = counts.iter().any(|&count| count == 3);
    (contains_two, contains_three)
}

fn compare_ids(id1: &str, id2: &str) -> Option<String> {
    let mut diff_index: Option<usize> = None;
    for (index, c) in id1.chars().enumerate() {
        if c != id2.chars().nth(index).expect("IDs of differing lengths") {
            if diff_index.is_some() {
                return None;
            } else {
                diff_index = Some(index);
            }
        }
    }
    diff_index.map(|index| id1[0..index].to_string() + &id1[index + 1..])
}