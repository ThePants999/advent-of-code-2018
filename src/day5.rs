pub fn day5(input_lines: &[String]) -> (String, String) {
    let part1 = react_polymer(input_lines[0].clone()).len().to_string();
    let mut part2 = input_lines[0].len();
    for removed_char in 'a'..='z' {
        let uppercase_char = removed_char.to_ascii_uppercase();
        let mut polymer = input_lines[0].clone();
        polymer.retain(|c| c != removed_char && c != uppercase_char);
        let new_len = react_polymer(polymer).len();
        if new_len < part2 {
            part2 = new_len;
        }
    }
    (part1, part2.to_string())
}

fn react_polymer(polymer: String) -> String {
    let mut remaining = polymer;
    loop {
        let mut elimination = false;
        let mut new_str = String::with_capacity(remaining.len());
        let mut prev_char: Option<char> = None;

        for char in remaining.chars() {
            if let Some(prev) = prev_char {
                if char.to_ascii_lowercase() == prev.to_ascii_lowercase() {
                    if char != prev {
                        // Characters are different, but the same when converted to lowercase - eliminate.
                        elimination = true;
                        prev_char = None;
                    } else {
                        // Characters are identical - keep prev.
                        new_str.push(prev);
                        prev_char = Some(char);
                    }
                } else {
                    // Characters are different letters - keep prev.
                    new_str.push(prev);
                    prev_char = Some(char);
                }
            } else {
                // We've just started or just eliminated. Remember this one and move on.
                prev_char = Some(char);                
            }
        }

        if let Some(prev) = prev_char {
            new_str.push(prev);
        }
        remaining = new_str;

        if !elimination {
            break;
        }
    }

    remaining
}