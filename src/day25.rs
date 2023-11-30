use std::collections::{HashMap, VecDeque};
use aoc::*;



pub fn from_snafu(input: &str) -> i64 {
    input.chars().fold(0, |acc, c| {
        (acc * 5) + match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!()
        }
    })
}

pub fn to_snafu(input: i64) -> String {
    if input == 0 {
        return String::new()
    }

    let rem = input % 5;
    let digit = ['0', '1', '2', '=', '-'][rem as usize];
    let new_dec = (input + 2) / 5;
    let mut snafu = to_snafu(new_dec);
    snafu.push(digit);
    snafu
}



#[aoc(day25, part1)]
pub fn part1(inputs: &str) -> String {
    to_snafu(inputs.lines().map(|l| from_snafu(l)).sum())
}


#[aoc(day25, part2)]
pub fn part2(inputs: &str) -> i64 {
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    pub fn test_from_snafu() {
        assert_eq!(from_snafu("1"), 1);
        assert_eq!(from_snafu("2"), 2);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("1-"), 4);
        assert_eq!(from_snafu("10"), 5);
        assert_eq!(from_snafu("11"), 6);
        assert_eq!(from_snafu("12"), 7);
        assert_eq!(from_snafu("2="), 8);
        assert_eq!(from_snafu("2-"), 9);
        assert_eq!(from_snafu("20"), 10);
        assert_eq!(from_snafu("1=0"), 15);
        assert_eq!(from_snafu("1-0"), 20);
        assert_eq!(from_snafu("1=11-2"), 2022);
        assert_eq!(from_snafu("1-0---0"), 12345);
        assert_eq!(from_snafu("1121-1110-1=0"), 314159265);

    }

    #[test]
    pub fn test_to_snafu() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), "2=-1=0");
    }

    #[test]
    pub fn test2() {
        //assert_eq!(part2(&generator(&SAMPLE)), 0);
    }
}
