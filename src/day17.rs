use std::collections::{HashSet, VecDeque};
use nalgebra::min;
use aoc::*;
use crate::day17::Shape::*;
use crate::day17::Jet::*;

const ROUNDS: usize = 2022;
const CHAMBER_WIDTH: i64 = 7;

type Data = Vec<Jet>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Jet {
    Left,
    Right,
}

impl Jet {
    pub fn to_offset(&self) -> Point {
        use Jet::*;
        match self {
            Left => LEFT,
            Right => RIGHT,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum Shape {
    Minus,
    Plus,
    L,
    Bar,
    Cluster,
    Border,
}

impl Shape {
    pub fn initial_position_x(&self) -> Point {
        use Shape::*;
        match self {
            Minus => [2, -3],
            Plus => [3, -3],
            L => [2, -3],
            Bar => [2, -3],
            Cluster => [2, -3],
            Border => [0, 0],
        }
    }

    pub fn occupied(&self, corner: &Point) -> Vec<Point> {
        match self {
            Minus => (0..4).into_iter().map(|xo| point_add(*corner, point_mul(RIGHT, xo))).collect(),
            Plus => {
                let mut v = Vec::with_capacity(5);
                v.push(*corner);
                v.push(point_add(*corner, UP));
                v.push(point_add(point_add(*corner, UP), LEFT));
                v.push(point_add(point_add(*corner, UP), RIGHT));
                v.push(point_add(point_add(*corner, UP), UP));
                v
            },
            L => {
                let mut v = Vec::with_capacity(5);
                v.push(*corner);
                v.push(point_add(*corner, RIGHT));
                let p = point_add(point_add(*corner, RIGHT), RIGHT);
                v.push(p);
                v.push(point_add(p, UP));
                v.push(point_add(point_add(p, UP), UP));
                v
            },
            Bar => (0..4).into_iter().map(|yo| point_add(*corner, point_mul(UP, yo))).collect(),
            Cluster => {
                let mut v = Vec::with_capacity(4);
                v.push(*corner);
                v.push(point_add(*corner, RIGHT));
                v.push(point_add(*corner, UP));
                v.push(point_add(point_add(*corner, UP), RIGHT));
                v
            },
            Border => (0..7).into_iter().map(|xo| point_add(*corner, point_mul(RIGHT, xo))).collect(),
        }
    }
}

const SHAPE_ORDER: [Shape; 5] = [Shape::Minus, Shape::Plus, Shape::L, Shape::Bar, Shape::Cluster];


#[aoc_generator(day17)]
pub fn generator(input: &str) -> Data {
    input.trim().chars().map(|c| {
        match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!(),
        }
    }).collect()
}

#[aoc(day17, part1)]
pub fn part1(inputs: &Data) -> i64 {
    let mut shapes = SHAPE_ORDER.iter().cycle();
    let mut jets = inputs.clone().iter().cycle();
    let mut miny = 0;
    let mut height = 0;
    let mut occupied = HashSet::<Point>::new();
    for p in Border.occupied(&Border.initial_position_x()).iter() {
        occupied.insert(*p);
    }

    let collides = |p: Point, s: &Shape, g: &HashSet<Point>| {
        s.occupied(&p).iter().any(|p|  p[0] < 0 || p[0] >= CHAMBER_WIDTH || g.contains(p))
    };

    for i in 0..ROUNDS {
        let shape = shapes.next().unwrap();
        //println!("===== {}: {:?} ===== {}", i, shape, miny);
        let mut p = [shape.initial_position_x()[0], miny - 1 + shape.initial_position_x()[1]];
        //println!("{:?}", p);
        loop {
            let j = jets.next().unwrap();
            let new_p = match j {
                Left => [p[0] - 1, p[1]],
                Right => [p[0] + 1, p[1]],
            };
            if !collides(new_p, shape, &occupied) {
                p = new_p;
                //println!("Jet: {:?}, {:?}", p, j);
            } else {
                //println!("Jet: No Move, {:?}", j);
            }

            let new_p = [p[0], p[1] + 1];
            height += 1;
            if !collides(new_p, shape, &occupied) {
                p = new_p;
                //println!("Down: {:?}", p);
            } else {
                //println!("Rest: {:?}", p);
                for x in shape.occupied(&p).iter() {
                    //print!("{:?} || ", x);
                    if x[1] < miny {
                        miny = x[1];
                    }
                    occupied.insert(*x);
                }
                //println!();
                break;
            }
        }

    }

    -miny
}


#[aoc(day17, part2)]
pub fn part2(inputs: &Data) -> i64 {
    0
}




#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 3068);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 1514285714288);
    }
}
