use std::collections::HashSet;
use aoc::*;
use regex::Regex;
use itertools::Itertools;

type Data = (Vec<(Point, i64)>, Vec<Point>, Vec<Point>);



#[aoc_generator(day18)]
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

#[aoc(day18, part1)]
pub fn part1(inputs: &Data) -> usize {
    0
}


#[aoc(day18, part2)]
pub fn part2(inputs: &Data) -> i64 {
    0
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
        assert_eq!(part1(&generator(&SAMPLE)), 0);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 0);
    }
}
