use std::collections::HashMap;
use aoc::*;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Structure {
    Empty,
    Rock,
    Sand
}


type Data = (HashMap::<Point, Structure>, i64);

pub const SAND_ORIGIN: Point = [500, 0];

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(",").unwrap();
    [x.parse().unwrap(), y.parse().unwrap()]
}

fn get_next_possibles(p: Point) -> [Point; 3] {
    return [
        [p[0], p[1]+1],
        [p[0]-1, p[1]+1],
        [p[0]+1, p[1]+1]
    ]
}


#[aoc_generator(day14)]
pub fn generator(input: &str) -> Data {
    let mut min_y = 0;
    (input.lines().fold(HashMap::new(), |mut acc, line| {
        let points: Vec<Point> = line.split(" -> ").map(|part| parse_point(part)).collect();
        points.windows(2).for_each(|p| {
            let (x1, x2) = (std::cmp::min(p[0][0], p[1][0]), std::cmp::max(p[0][0], p[1][0]));
            let (y1, y2) = (std::cmp::min(p[0][1], p[1][1]), std::cmp::max(p[0][1], p[1][1]));
            for x in x1..=x2 {
                for y in y1..=y2 {
                    acc.insert([x, y], Structure::Rock);
                    if y > min_y {
                        min_y = y;
                    }
                }
            }
        });
        acc
    }), min_y + 2)
}

pub fn print(map: &HashMap<Point, Structure>) {
    for y in 0..=12 {
        for x in 485..=515 {
            if (x, y) == (500, 0) {
                print!("+");
            } else {
                match map.get(&[x, y]) {
                    Some(Structure::Sand) => print!("o"),
                    Some(Structure::Rock) => print!("#"),
                    _ => print!("."),
                }
            }
        }
        println!();
    }
    println!();
}

#[aoc(day14, part1)]
pub fn part1(inputs: &Data) -> usize {
    let mut counter = 0;
    let mut map = inputs.0.clone();
    let min_y = inputs.1;
    'outer: loop {
        let mut pos = SAND_ORIGIN;
        loop {
            if let Some(next) = get_next_possibles(pos).iter().find_map(|next| match map.get(next) {
                Some(Structure::Empty) | None => Some(next),
                _ => None
            }) {
                if next[1] > min_y {
                    return counter;
                }
                pos = *next;
            } else {
                map.insert(pos, Structure::Sand);
                counter += 1;
                continue 'outer;
            }
        }
    }
}


#[aoc(day14, part2)]
pub fn part2(inputs: &Data) -> usize {
    let mut counter = 0;
    let mut map = inputs.0.clone();
    let min_y = inputs.1;
    'outer: loop {
        let mut pos = SAND_ORIGIN;
        loop {
            if let Some(next) = get_next_possibles(pos).iter().find_map(|next| match map.get(next) {
                Some(Structure::Empty) | None => Some(next),
                _ => None
            }) {
                if next[1] == min_y - 1 {
                    map.insert(*next, Structure::Sand);
                    counter += 1;
                    continue 'outer;
                }
                pos = *next;
            } else {
                if map.get(&[500,0]).unwrap_or(&Structure::Empty) == &Structure::Sand {
                    return counter;
                }
                map.insert(pos, Structure::Sand);
                counter += 1;
                continue 'outer;
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 24);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 93);
    }
}
