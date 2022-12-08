use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use aoc::{Grid, point_add, point_mul};

type Data = Vec<Vec<i8>>;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Data {
    aoc::parse_grid_to(input.lines().collect::<Vec<_>>().as_slice(), |x| x.to_digit(10).unwrap() as i8)
}

#[aoc(day8, part1)]
pub fn part1(inputs: &Data) -> usize {
    inputs.points().map(|p| {
        let size = inputs.get_value(p).unwrap();
        aoc::DIRECTIONS.into_iter().map(|d| {
            (0..).into_iter().fold_while(0, |_, n| {
                return if let Some(other) = inputs.get_value(point_add(p, point_mul(d, n + 1))) {
                    if other >= size {
                        return Done(0)
                    }
                    Continue(0)
                } else {
                    Done(1)
                }
            }).into_inner()
        }).any(|vis| vis == 1)
    }).filter(|&b| b).count()
}

#[aoc(day8, part2)]
pub fn part2(inputs: &Data) -> i32 {
    inputs.points().map(|p| {
        let size = inputs.get_value(p).unwrap();
        aoc::DIRECTIONS.into_iter().map(|d| {
            (0..).into_iter().fold_while(0, |dist, n| {
                return if let Some(other) = inputs.get_value(point_add(p, point_mul(d, n + 1))) {
                    if other >= size {
                        return Done(dist + 1)
                    }
                    Continue(dist + 1)
                } else {
                    Done(dist)
                }
            }).into_inner()
        }).product()
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 21)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 8)
    }
}
