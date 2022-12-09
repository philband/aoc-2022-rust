use std::collections::HashSet;
use itertools::Itertools;
use aoc::{neighbors, neighbors_incl_diagonals, Point, point_add};

type Data = Vec<(Point, i32)>;

const DEBUG: bool = false;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<_>>();
            let d = match parts[0].chars().nth(0).unwrap() {
                'R' => aoc::RIGHT,
                'L' => aoc::LEFT,
                'U' => aoc::UP,
                'D' => aoc::DOWN,
                _ => unreachable!()
            };
            (d, parts[1].parse().unwrap())
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(inputs: &Data) -> usize {
    run(inputs, 1)
}


#[aoc(day9, part2)]
pub fn part2(inputs: &Data) -> usize {
    run(inputs, 9)
}

pub fn run(inputs: &Data, tail_len: usize) -> usize {
    let mut head: Point = [0,0];
    let mut tails: Vec<Point> = vec![[0,0]; tail_len];
    let tail_count = tails.len();
    let visited = HashSet::<Point>::new();


    inputs.into_iter().fold(visited, |visited, &(direction, distance)| {
        let ret = (0..distance).into_iter().fold(visited, |mut visited, _| {
            head = point_add(head, direction);
            (0..tail_len).into_iter().for_each(|i| {
                let current_head = if i == 0 {
                    head
                } else {
                    tails[i-1]
                };
                let current_tail = &mut tails[i];

                if !neighbors_incl_diagonals(current_head).contains(current_tail) && !current_head.eq(current_tail) {
                    if i == tail_count-1 {
                        visited.insert(*current_tail);
                    }
                    if let Some(ht) = neighbors(*current_tail).find(|ht| neighbors(current_head).any(|hn| hn.eq(ht))) {
                        *current_tail = ht;
                    } else if let Some(ht) = neighbors_incl_diagonals(*current_tail).find(|ht| neighbors(current_head).any(|hn| hn.eq(ht))) {
                        *current_tail = ht;
                    } else if let Some(ht) = neighbors_incl_diagonals(*current_tail).find(|ht| neighbors_incl_diagonals(current_head).any(|hn| hn.eq(ht))) {
                        *current_tail = ht;
                    }
                }
            });

            if DEBUG {
                for y in -15..=6 {
                    for x in -11..=15 {
                        if [x, y] == head {
                            print!("H");
                        } else {
                            match (0..tail_len-1).position(|i| tails[i] == [x, y]) {
                                Some(i) => print!("{}", i+1),
                                _ => print!(".")
                            }
                        }
                    }
                    println!()
                }
                println!();
            }

            if tails[tail_len-1] != [0,0] {
                visited.insert(tails[tail_len-1]);
            }
            visited
        });

        ret
    }).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";


    const SAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 13);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 0);
        assert_eq!(part2(&generator(&SAMPLE2)), 36);
    }
}
