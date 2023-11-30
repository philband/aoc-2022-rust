use std::collections::HashMap;
use aoc::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Elve {
    proposed: Option<Point>,
    directions: [Point; 4]
}

impl Default for Elve {
    fn default() -> Self {
        Elve {
            proposed: None,
            directions: [NORTH, SOUTH, WEST, EAST],
        }
    }
}

pub trait ElveMovement {
    fn round(&mut self) -> usize;
    fn propose(&mut self);
    fn mv(&mut self) -> usize;
    fn empty_rectiles(&self) -> i64;
    fn print(&self);
}

impl ElveMovement for Data {
    fn round(&mut self) -> usize {
        self.propose();
        return self.mv()
    }

    fn propose(&mut self) {
        let map = self.clone();
        'outer: for (p, mut e) in self.iter_mut() {
            if neighbors_incl_diagonals(*p).all(|x| map.get(&x).is_none()) {
                e.proposed = None;
                continue 'outer
            }
            for d in e.directions.iter() {
                let to_check = match *d {
                    NORTH => [NORTH_EAST, NORTH, NORTH_WEST],
                    SOUTH => [SOUTH_WEST, SOUTH, SOUTH_EAST],
                    WEST => [NORTH_WEST, WEST, SOUTH_WEST],
                    EAST => [NORTH_EAST, EAST, SOUTH_EAST],
                    _ => unreachable!(),
                };
                if to_check.iter().map(|delta| map.get(&point_add(*p, *delta))).all(|x| x.is_none()) {
                    e.proposed = Some(point_add(*p, *d));
                    continue 'outer
                }
            }
            e.proposed = None;
        }
    }

    fn mv(&mut self) -> usize {
        let mut moved = 0;
        let map = self.clone();
        let targets: HashMap<Point, usize> = map
            .iter()
            .filter(|(_p, e)| e.proposed.is_some())
            .fold(HashMap::new(), |mut map, (_origin, e) | {
                *map.entry(e.proposed.unwrap()).or_insert(0) += 1;
                map
            });

        for (from, _e) in map.iter().filter(|(_, e)| e.proposed.is_some() && *targets.get(&e.proposed.unwrap()).unwrap() == 1) {
            moved += 1;
            let mut e = self.remove(from).unwrap();
            let to = e.proposed.unwrap();
            e.proposed = None;
            self.insert(to, e);
        }

        for (_, mut e) in self.iter_mut() {
            if e.proposed.is_some() {
                e.proposed = None;
            }
            e.directions.rotate_left(1)
        }
        moved
    }

    fn empty_rectiles(&self) -> i64 {
        let minx = self.iter().map(|(p, _)| p[0]).min().unwrap();
        let maxx = self.iter().map(|(p, _)| p[0]).max().unwrap();
        let miny = self.iter().map(|(p, _)| p[1]).min().unwrap();
        let maxy = self.iter().map(|(p, _)| p[1]).max().unwrap();
        let elves = self.iter().count() as i64;
        ((maxx - minx + 1) * (maxy - miny + 1)) - elves
    }

    fn print(&self) {
        for y in (-11..=0).rev() {
            for x in 0..14 {
                if self.get(&[x, y]).is_some() {
                    print!("#")
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

type Data = HashMap<Point, Elve>;


#[aoc_generator(day23)]
pub fn generator(input: &str) -> Data {
    input.lines().enumerate().fold(HashMap::new(), |map, (negy, line)| {
        line.chars().enumerate().fold(map, |mut map, (x, c)| {
            if c == '#' {
                map.insert([x as i64, -(negy as i64)], Elve::default());
            }
            map
        })
    })
}



#[aoc(day23, part1)]
pub fn part1(inputs: &Data) -> i64 {
    let mut map = inputs.clone();
    for _i in 0..10 {
        map.round();
    }
    map.empty_rectiles()
}


#[aoc(day23, part2)]
pub fn part2(inputs: &Data) -> i32 {
    let mut map = inputs.clone();
    for i in 1.. {
        if map.round() == 0 {
            return i
        }
    }
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 110);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 20);
    }
}
