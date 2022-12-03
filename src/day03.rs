use itertools::Itertools;

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
    inputs.iter().tuples::<(_, _, _)>().map(|(a, b, c)| {
        let mut joined = a.0.to_string();
        joined.push_str(a.1.as_str());
        priority(joined.chars().into_iter().find(|x|
            (b.0.contains(*x) || b.1.contains(*x)) &&
            (c.0.contains(*x) || c.1.contains(*x))
        ).unwrap())
    }).sum()
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
