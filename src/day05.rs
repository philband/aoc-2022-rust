use std::collections::VecDeque;
use regex::Regex;

type Statement = (usize, usize, usize);
type Data = ([VecDeque::<char>; 9], Vec::<Statement>);


#[aoc_generator(day5)]
pub fn generator(input: &str) -> Data {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let stacks = lines.iter().take(8).fold([VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new(), VecDeque::<char>::new()], |mut acc, &l| {
        let chs: Vec<char> = l.chars().collect();
        (1..=9).into_iter().for_each(|n| {
            let c = chs[n * 4 - 3];
            if c != ' ' {
                acc[n-1].push_back(c)
            }
        });
        acc
    });
    let stmt: Vec::<(usize, usize, usize)> = lines.iter().skip(10).map(|l| {
        let captures = re.captures(l.trim()).expect("No match found");
        (captures[1].parse().unwrap(), captures[2].parse().unwrap(), captures[3].parse().unwrap())
    }).collect();
    (stacks, stmt)
}

#[aoc(day5, part1)]
pub fn part1(inputs: &Data) -> String {
    let (mut stacks, stmts) = inputs.clone();
    stmts.iter().for_each(|s| {
        let (n, from, to) = s;
        (0..*n).for_each(|_| {
            let elem = stacks[*from-1].pop_front().unwrap();
            stacks[*to-1].push_front(elem);
        })

    });
    stacks.iter_mut().fold(String::new(), |mut acc, s| {
        acc.push(s.pop_front().unwrap());
        acc
    })
}

#[aoc(day5, part2)]
pub fn part2(inputs: &Data) -> String {
    let (mut stacks, stmts) = inputs.clone();
    stmts.iter().for_each(|s| {
        let (n, from, to) = s;

        let items: Vec<char> = (0..*n).into_iter().map(|_| {
            stacks[*from-1].pop_front().unwrap()
        }).collect();
        items.iter().rev().for_each(|i| {
            stacks[*to-1].push_front(*i);
        });
    });
    stacks.iter_mut().fold(String::new(), |mut acc, s| {
        acc.push(s.pop_front().unwrap());
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), "")
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), "")
    }
}
