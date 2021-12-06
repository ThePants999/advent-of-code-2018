use std::collections::{BTreeSet, BTreeMap, HashSet};

const NUM_STEPS: usize = 26;
const FIRST_STEP: char = 'A';

pub fn day7(input_lines: &[String]) -> (String, String) {
    let mut steps: Vec<Step> = Vec::with_capacity(NUM_STEPS);
    for id in 0..NUM_STEPS {
        steps.push(Step::new(id));
    }

    for line in input_lines {
        let prereq = (line.chars().nth(5).unwrap() as usize) - (FIRST_STEP as usize);
        let step = (line.chars().nth(36).unwrap() as usize) - (FIRST_STEP as usize);
        steps[step].prereqs.insert(prereq);
    }

    // Part 1
    let mut order = String::new();
    let mut done_steps: HashSet<usize> = HashSet::with_capacity(NUM_STEPS);
    while order.len() < NUM_STEPS {
        for step in steps.iter_mut() {
            if step.status != Status::Done && done_steps.is_superset(&step.prereqs) {
                // All prereqs complete, perform this step and restart.
                step.status = Status::Done;
                done_steps.insert(step.id);
                order.push((FIRST_STEP as u8 + step.id as u8) as char);
                break;
            }
        }
    }
    let part1 = order;

    // Part 2
    // Reset step status
    for step in steps.iter_mut() {
        step.status = Status::NotStarted;
    }

    let mut order = String::new();
    let mut done_steps: HashSet<usize> = HashSet::with_capacity(NUM_STEPS);
    let mut available_workers = 5;
    let mut task_completion: BTreeMap<u64, BTreeSet<usize>> = BTreeMap::new();
    let mut current_time = 0u64;

    while order.len() < NUM_STEPS {
        // Start as many steps as currently possible.
        for step in steps.iter_mut() {
            if step.status == Status::NotStarted && done_steps.is_superset(&step.prereqs) && available_workers > 0 {
                // We're ready to start this step. Kick it off.
                let finish_time = current_time + step.id as u64 + 61;
                let set = task_completion.entry(finish_time).or_insert_with(BTreeSet::new);
                set.insert(step.id);
                step.status = Status::InProgress;
                available_workers -= 1;
            }
        }

        // Wait for the first step(s) to complete.
        let (time, set) = task_completion.iter().next().unwrap();
        current_time = *time;
        for id in set.iter() {
            done_steps.insert(*id);
            order.push((FIRST_STEP as u8 + *id as u8) as char);
            steps[*id].status = Status::Done;
            available_workers += 1;
        }
        task_completion.remove(&current_time);
    }
    let part2 = current_time.to_string();

    (part1, part2)
}

#[derive(PartialEq)]
enum Status {
    NotStarted,
    InProgress,
    Done,
}

struct Step {
    id: usize,
    prereqs: HashSet<usize>,
    status: Status,
}

impl Step {
    fn new(id: usize) -> Self {
        Self {
            id,
            prereqs: HashSet::new(),
            status: Status::NotStarted,
        }
    }
}