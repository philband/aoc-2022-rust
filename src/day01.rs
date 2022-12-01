use itertools::Itertools;

type Data = Vec<i32>;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Data {
    input
        .split("\n\n")
        .map(|part| part
            .lines()
            .map(|l| l.parse::<i32>().unwrap()).sum()
        ).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &Data) -> i32 {
    *inputs.into_iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(inputs: &Data) -> i32 {
    inputs.iter().sorted().rev().take(3).sum()
}
#[cfg(test)]
mod tests {
    //use super::*;


    use crate::day01::*;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 24000)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 45000)
    }
}
