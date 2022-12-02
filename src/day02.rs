use itertools::Itertools;

type Data = Vec<(char, char)>;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Data {
    input.lines().map(|l| {
        (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap())
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &Data) -> i32 {
    inputs
        .iter()
        .map(|(o, p)| {
            let res = match (o, p) {
                ('A', 'Y') => 6,
                ('B', 'Z') => 6,
                ('C', 'X') => 6,
                ('A', 'X') => 3,
                ('B', 'Y') => 3,
                ('C', 'Z') => 3,
                _ => 0
            };
            let choice = match p {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => unreachable!()
            };
            res + choice
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &Data) -> i32 {
    inputs
        .iter()
        .map(|(o, p)| {
            let n = match(o, p) {
                ('A', 'X') => 'Z',
                ('B', 'X') => 'X',
                ('C', 'X') => 'Y',
                ('A', 'Y') => 'X',
                ('B', 'Y') => 'Y',
                ('C', 'Y') => 'Z',
                ('A', 'Z') => 'Y',
                ('B', 'Z') => 'Z',
                ('C', 'Z') => 'X',
                _ => unreachable!()
            };
            let res = match (o, n) {
                ('A', 'Y') => 6,
                ('B', 'Z') => 6,
                ('C', 'X') => 6,
                ('A', 'X') => 3,
                ('B', 'Y') => 3,
                ('C', 'Z') => 3,
                _ => 0
            };
            let choice = match n {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => unreachable!()
            };
            res + choice
        })
        .sum()
}
#[cfg(test)]
mod tests {
    //use super::*;


    use crate::day02::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 15)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 12)
    }
}
