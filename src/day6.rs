use itertools::Itertools;

pub fn day6(input_lines: &[String]) -> (String, String) {
    let mut locations: Vec<Location> = input_lines.iter().map(|line| Location::parse(line)).collect();

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (1000isize, 1000isize, 0isize, 0isize);
    for loc in locations.iter() {
        if loc.x < min_x { min_x = loc.x; }
        if loc.y < min_y { min_y = loc.y; }
        if loc.x > max_x { max_x = loc.x; }
        if loc.y > max_y { max_y = loc.y; }
    }
    for loc in locations.iter_mut() {
        if (loc.x == min_x) || (loc.x == max_x) || (loc.y == min_y) || (loc.y == max_y) {
            loc.set_external();
        }
    }

    let mut num_within_region = 0u64;
    for (x, y) in (min_x..=max_x).cartesian_product(min_y..=max_y) {
        let mut total_distance = 0isize;
        let mut shortest_distance = 1000isize;
        let mut closest_location: Option<&mut Location> = None;
        for loc in locations.iter_mut() {
            let distance = loc.distance_to(x, y);
            total_distance += distance;
            #[allow(clippy::comparison_chain)]
            if distance < shortest_distance {
                shortest_distance = distance;
                closest_location = Some(loc);
            } else if distance == shortest_distance {
                // Second or subsequent equidistant location found.
                closest_location = None;
            }
        }
        if let Some(loc) = closest_location {
            loc.space_is_closest();
        }
        if total_distance < 10000 {
            num_within_region += 1;
        }
    }

    let mut max_num_closest = 0u64;
    for loc in locations.iter() {
        if loc.num_closest > max_num_closest { max_num_closest = loc.num_closest; }
    }

    let part1 = max_num_closest.to_string();
    let part2 = num_within_region.to_string();
    (part1, part2)
}

struct Location {
    x: isize,
    y: isize,
    num_closest: u64,
    external: bool,
}

impl Location {
    fn parse(line: &str) -> Self {
        let mut coords = line.split(", ");
        Self {
            x: coords.next().unwrap().parse::<isize>().expect("Input not numeric"),
            y: coords.next().unwrap().parse::<isize>().expect("Input not numeric"),
            num_closest: 0,
            external: false,
        }
    }

    fn distance_to(&self, x: isize, y: isize) -> isize {
        (x - self.x).abs() + (y - self.y).abs()
    }

    fn space_is_closest(&mut self) {
        self.num_closest += 1;
    }

    fn set_external(&mut self) {
        self.external = true;
    }
}