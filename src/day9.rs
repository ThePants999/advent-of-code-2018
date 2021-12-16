use std::collections::{LinkedList, linked_list::CursorMut};

// ************************
// REQUIRES NIGHTLY RUST
// due to use of LinkedList cursor
// ************************

const NUM_PLAYERS: usize = 423;
const LAST_MARBLE: u64 = 71944;

pub fn day9(_input_lines: &[String]) -> (String, String) {
    let mut circle: LinkedList<u64> = LinkedList::new();
    circle.push_back(0);
    let mut current = circle.cursor_back_mut();
    
    let mut scores: [u64; NUM_PLAYERS + 1] = [0; NUM_PLAYERS + 1];
    let mut current_player = 1;

    for marble in 1..=LAST_MARBLE {
        play_round(&mut current, &mut scores, &mut current_player, marble);
    }
    
    let part1 = scores.iter().max().unwrap().to_string();

    for marble in LAST_MARBLE+1..=LAST_MARBLE*100 {
        play_round(&mut current, &mut scores, &mut current_player, marble);
    }
    
    let part2 = scores.iter().max().unwrap().to_string();

    (part1, part2)
}

fn play_round(cursor: &mut CursorMut<u64>, scores: &mut [u64], current_player: &mut usize, marble: u64) {
    if marble % 23 == 0 {
        scores[*current_player] += marble;
        seek_cursor(cursor, -7);
        scores[*current_player] += cursor.remove_current().unwrap();
    } else {
        seek_cursor(cursor, 1);
        cursor.insert_after(marble);
        cursor.move_next();
    }

    *current_player += 1;
    if *current_player == NUM_PLAYERS {
        *current_player = 0;
    }
}

fn seek_cursor<T>(cursor: &mut CursorMut<T>, mut steps: isize) {
    let forwards = steps >= 0;
    let step_fn = if forwards {
        CursorMut::move_next
    } else {
        steps *= -1;
        CursorMut::move_prev
    };
    for _ in 0..steps {
        step_fn(cursor);
        if cursor.current().is_none() {
            step_fn(cursor);
        }
    }
}