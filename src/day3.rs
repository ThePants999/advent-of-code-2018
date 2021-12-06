use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

const FABRIC_SIZE: usize = 1000;

struct Claim {
    id: usize,
    min_x: usize,
    min_y: usize,
    width: usize,
    height: usize,
}

struct Fabric {
    all_claims: HashSet<usize>,
    location_claims: Vec<Vec<HashSet<usize>>>,
}

impl Fabric {
    fn new() -> Self {
        let mut location_claims: Vec<Vec<HashSet<usize>>> = Vec::with_capacity(FABRIC_SIZE);
        for _ in 0..FABRIC_SIZE {
            let mut row_claims: Vec<HashSet<usize>> = Vec::with_capacity(FABRIC_SIZE);
            for _ in 0..FABRIC_SIZE {
                row_claims.push(HashSet::new());
            }
            location_claims.push(row_claims);
        }
        Self {
            all_claims: HashSet::new(),
            location_claims,
        }
    }

    fn process_claim(&mut self, claim: &Claim) {
        self.all_claims.insert(claim.id);

        for x in claim.min_x..claim.min_x+claim.width {
            for y in claim.min_y..claim.min_y+claim.height {
                self.location_claims[x][y].insert(claim.id);

                if self.location_claims[x][y].len() > 1 {
                    let claims_to_remove = self.location_claims[x][y].clone();
                    self.all_claims.retain(|claim| !claims_to_remove.contains(claim));
                }
            }
        }    
    }
}

pub fn day3(input_lines: &[String]) -> (String, String) {
    let mut fabric = Fabric::new();
    input_lines.iter().map(|line| parse_claim(line)).for_each(|claim| fabric.process_claim(&claim));

    let mut intersections = 0u64;
    for (x, y) in (0..FABRIC_SIZE).cartesian_product(0..FABRIC_SIZE) {
        if fabric.location_claims[x][y].len() > 1 {
            intersections += 1;
        }
    }

    (intersections.to_string(),fabric.all_claims.iter().next().unwrap().to_string())
}

fn parse_claim(input: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }
    let caps = RE.captures(input).expect("Invalid input");
    let id = caps[1].parse::<usize>().expect("Non-numeric ID");
    let min_x = caps[2].parse::<usize>().expect("Non-numeric X");
    let min_y = caps[3].parse::<usize>().expect("Non-numeric Y");
    let width = caps[4].parse::<usize>().expect("Non-numeric width");
    let height = caps[5].parse::<usize>().expect("Non-numeric height");
    Claim { id, min_x, min_y, width, height }
}
