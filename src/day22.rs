use std::collections::HashMap;
use aoc::*;


pub enum Pieces {
    OutOfMap,
    Empty,
    Wall,
}

pub enum Instruction {
    Move(usize),
    L,
    R,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    pub fn turn(self, i: &Instruction) -> Self {
        use Instruction::*;
        match i {
            L => self.left(),
            R => self.right(),
            _ => self,
        }
    }

    fn left(&self) -> Self  {
        use Direction::*;
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn right(self) -> Self {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn walk(self, p: Point) -> Point {
        use Direction::*;
        match self {
            Right => point_add(p, [1, 0]),
            Left => point_add(p, [-1, 0]),
            Down => point_add(p, [0, 1]),
            Up => point_add(p, [0, -1]),
        }
    }
}

pub struct Player {
    position: Point,
    direction: Direction,
}

impl Player {
    pub fn execute(&mut self, map: &HashMap<Point, Pieces>, i: &Instruction) {
        use Instruction::*;
        use Pieces::*;
        use Direction::*;
        if let Move(n) = i {
            for i in 0..*n {
                let new_p = self.direction.walk(self.position);
                match map.get(&new_p) {
                    Some(Wall) => { break; }
                    Some(Empty) => { self.position = new_p; }
                    Some(OutOfMap) | None => {
                        let mut p = self.position;
                        match self.direction {
                            Up => {
                                let max_y = map.iter().map(|(p, _)| p[1]).max().unwrap() + 1;
                                p = [self.position[0], max_y];
                            },
                            Down => {
                                let min_y = map.iter().map(|(p, _)| p[1]).min().unwrap() - 1;
                                p = [self.position[0], min_y];
                            },
                            Left => {
                                let max_x = map.iter().map(|(p, _)| p[0]).max().unwrap() + 1;
                                p = [max_x, self.position[1]];
                            }
                            Right => {
                                let min_x = map.iter().map(|(p, _)| p[0]).min().unwrap() - 1;
                                p = [min_x, self.position[1]];
                            }
                        }
                        loop {
                            match map.get(&p) {
                                Some(Wall) => { break; },
                                Some(Empty) => { self.position = p; break; }
                                Some(OutOfMap) | None => ()
                            };
                            p = self.direction.walk(p)
                        }
                    }
                }
            }
        } else {
            self.direction = self.direction.turn(i);
        }
    }
}


type Data = (HashMap<Point, Pieces>, Vec<Instruction>);


#[aoc_generator(day22)]
pub fn generator(input: &str) -> Data {
    let mut parts = input.split("\n\n");
    let map = parts.next().unwrap();
    let ins = parts.next().unwrap();
    let board = map.lines().enumerate().fold(HashMap::new(), |acc, (y, line)| {
        line.chars().enumerate().into_iter().fold(acc, |mut acc, (x, c)| {
            match c {
                ' ' => (),
                '.' => { acc.insert([(x as i64) + 1, (y as i64) + 1], Pieces::Empty); },
                '#' => { acc.insert([(x as i64) + 1, (y as i64) + 1], Pieces::Wall); },
                _ => unreachable!()
            }
            acc
        })
    });

    let instructions = ins.split_inclusive(['R', 'L']).fold(Vec::new(), |mut acc, i| {
        if let Ok(i) = i[0..i.len()-1].parse::<usize>() {
            acc.push(Instruction::Move(i))
        }
        if let Some(i) = match i.chars().last().unwrap() {
            'L' => Some(Instruction::L),
            'R' => Some(Instruction::R),
            _ => None
        } {
            acc.push(i);
        }
        acc
    });


    (board, instructions)
}


#[aoc(day22, part1)]
pub fn part1(inputs: &Data) -> i64 {
    let (board, ins) = inputs;
    let start = (1..).into_iter().map(|x| [x, 1]).filter_map(|p| {
        match board.get(&p) {
            Some(Pieces::Empty) => Some(p),
            _ => None
        }
    }).next().unwrap();
    let mut p = Player {
        direction: Direction::Right,
        position: start,
    };
    for i in ins {
        p.execute(board, i);
    }
    p.position[1] * 1000 + p.position[0] * 4 + p.direction as i64
}


#[aoc(day22, part2)]
pub fn part2(inputs: &Data) -> i64 {
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 6032);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 0);
    }
}
