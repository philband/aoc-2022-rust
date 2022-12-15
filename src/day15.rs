use std::collections::HashSet;
use aoc::*;
use regex::Regex;
use itertools::Itertools;

type Data = (Vec<(Point, i64)>, Vec<Point>, Vec<Point>);



#[aoc_generator(day15)]
pub fn generator(input: &str) -> Data {
    let mut sensors = vec![];
    let mut beacons = vec![];
    let mut pos_dist = vec![];
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    for line in input.lines() {
        let c = re.captures(line).expect("No match found");
        let sensor: Point = [c[1].parse().unwrap(), c[2].parse().unwrap()];
        let beacon: Point = [c[3].parse().unwrap(), c[4].parse().unwrap()];
        let dist = manhattan(sensor, beacon);
        sensors.push(sensor);
        beacons.push(beacon);
        pos_dist.push((sensor, dist))
    }
    (pos_dist, sensors, beacons)
}

#[aoc(day15, part1)]
pub fn part1(inputs: &Data) -> usize {
    solve_part1(inputs, 2000000)
}

#[aoc(day15, part1, naive)]
pub fn part1_naive(inputs: &Data) -> usize {
    solve_part1_naive(inputs, 2000000)
}


#[aoc(day15, part2)]
pub fn part2(inputs: &Data) -> i64 {
    solve_part2(inputs, 4000000)
}

pub fn solve_part1_naive(inputs: &Data, roi_y: i64) -> usize {
    let (pos_dist, sensors, beacons) = inputs;
    let mut impossibles: HashSet<Point> = HashSet::new();
    sensors.iter().filter(|p| p[1] == roi_y).for_each(|p| { impossibles.insert(*p); });
    for x in -5000000..=5000000 {
        let candidate = [x, roi_y];
        if beacons.contains(&candidate) {
            continue;
        }
        if pos_dist.iter().any(|(p, d)| manhattan(*p, candidate) <= *d) {
            impossibles.insert(candidate);
        }
    }
    impossibles.iter().count()
}



pub fn solve_part1(inputs: &Data, roi_y: i64) -> usize {
    let (pos_dist, _, beacons) = inputs;
    pos_dist
        .iter()
        .flat_map(|(p, d)| manhattan_circumference_contains_y(p, *d, roi_y))
        .unique()
        .filter(|p| !beacons.contains(p))
        .count()
}

pub fn solve_part2(inputs: &Data, roi: i64) -> i64 {
    let (pos_dist, _, beacons) = inputs;
    let uncovered = pos_dist
        .iter()
        .flat_map(|(p, d)| manhattan_circumference_plus(p, *d, 1))
        .filter(|p| p[0] > 0 && p[1] > 0 && p[0] <= roi && p[1] <= roi)
        .filter(|p| !beacons.contains(p))
        .find(|candidate| pos_dist.iter().all(|(p, d)| manhattan(*p, *candidate) > *d))
        .unwrap();
        uncovered[0] * 4000000 + uncovered[1]
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";


    #[test]
    pub fn test1() {
        assert_eq!(solve_part1(&generator(&SAMPLE), 10), 26);
    }

    #[test]
    pub fn test1_naive() {
        assert_eq!(solve_part1_naive(&generator(&SAMPLE), 10), 26);
    }

    #[test]
    pub fn test2() {
        assert_eq!(solve_part2(&generator(&SAMPLE), 20), 56000011);
    }
}
