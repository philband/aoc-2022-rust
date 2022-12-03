use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;

type Data = Vec<(String, String)>;

pub fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u8 - 'a' as u8 + 1) as i32
    } else {
        (c as u8 - 'A' as u8 + 27) as i32
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            (a.to_string(), b.to_string())
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &Data) -> i32 {
    inputs
        .iter()
        .map(|(a, b)| {
            priority(a.chars().find(|x| b.contains(*x)).unwrap())
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &Data) -> i32 {
    let count = inputs.len() / 3;
    (0..count)
        .into_iter()
        .map(|n| {
            priority(
                *inputs
                    .iter()
                    .skip(n * 3)
                    .take(3)
                    .enumerate()
                    .fold(
                        HashMap::<char, (usize, usize, usize)>::new(),
                        |mut acc, (i, (a, b))| {
                            let mut joined = a.to_string();
                            joined.push_str(b);
                            joined.chars().into_iter().fold(acc, |mut acc, c| {
                                let cnt = acc.entry(c).or_insert((0, 0, 0));
                                match i {
                                    0 => cnt.0 += 1,
                                    1 => cnt.1 += 1,
                                    2 => cnt.2 += 1,
                                    _ => unreachable!(),
                                }
                                acc
                            })
                        },
                    )
                    .iter()
                    .find(|(_, &(a, b, c))| a >= 1 && b >= 1 && c >= 1)
                    .unwrap()
                    .0,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 157)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 70)
    }
}
