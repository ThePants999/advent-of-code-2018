use std::collections::HashMap;

use regex::Regex;

pub fn day4(input_lines: &[String]) -> (String, String) {
    let guards = parse_input(input_lines.to_vec());

    let (most_asleep_guard_id, most_asleep_guard) = guards.iter().max_by_key(|(_, guard)| guard.asleep_total).unwrap();
    let (most_asleep_minute, _) = most_asleep_guard.asleep_count.iter().enumerate().max_by_key(|(_, &count)| count).unwrap();
    let part1 = (most_asleep_guard_id * most_asleep_minute as u64).to_string();

    let mut record = Record {
        guard_id: 0,
        minute: 0,
        asleep_count: 0,
    };
    for (guard_id, guard) in guards.iter() {
        for minute in 0..60 {
            if guard.asleep_count[minute] > record.asleep_count {
                record = Record {
                    guard_id: *guard_id,
                    minute,
                    asleep_count: guard.asleep_count[minute]
                }
            }
        }
    }
    let part2 = (record.guard_id * record.minute as u64).to_string();

    (part1, part2)
}

fn parse_input(mut input_lines: Vec<String>) -> HashMap<u64, Guard> {
    input_lines.sort();
    lazy_static! {
        static ref MAIN_REGEX: Regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.+)$").unwrap();
        static ref GUARD_REGEX: Regex = Regex::new(r"#(\d+)").unwrap();
    }

    let mut guards: HashMap<u64, Guard> = HashMap::new();
    let mut current_guard: Option<&mut Guard> = None;

    for line in input_lines {
        let caps = MAIN_REGEX.captures(&line).expect("Invalid input");
        let minute = caps[1].parse::<usize>().unwrap();
        match caps[2].chars().next().unwrap() {
            'G' => {
                let subcaps = GUARD_REGEX.captures(&caps[2]).unwrap();
                let guard_id = subcaps[1].parse::<u64>().unwrap();
                current_guard = Some(guards.entry(guard_id).or_insert_with(Guard::new));
            },
            'f' => {
                current_guard.as_mut().expect("Unspecified guard fell asleep!").record_asleep(minute);
            },
            'w' => {
                current_guard.as_mut().expect("Unspecified guard woke up!").record_awake(minute);
            },
            _ => unreachable!()
        }
    }

    guards
}

#[derive(Debug)]
struct Guard {
    asleep_count: [u64; 60],
    asleep_total: usize,
    last_recorded_asleep: Option<usize>,
}

impl Guard {
    fn new() -> Self {
        Self {
            asleep_count: [0; 60],
            asleep_total: 0,
            last_recorded_asleep: None,
        }
    }

    fn record_asleep(&mut self, minute: usize) {
        assert!(self.last_recorded_asleep.is_none());
        self.last_recorded_asleep = Some(minute);
    }

    fn record_awake(&mut self, minute: usize) {
        let first_minute = self.last_recorded_asleep.expect("Guard reported awake when not asleep");
        self.asleep_total += minute - first_minute;
        for time in first_minute..minute {
            self.asleep_count[time] += 1;
        }
        self.last_recorded_asleep = None;
    }
}

struct Record {
    guard_id: u64,
    minute: usize,
    asleep_count: u64,
}