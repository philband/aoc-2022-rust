use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Eq, Debug)]
pub enum Packet {
    Int(u32),
    List(Vec<Self>)
}

impl Packet {
    pub fn as_slice(&self) -> &[Self] {
        if let Self::List(list) = self {
            list.as_slice()
        } else {
            std::slice::from_ref(self)
        }
    }

    pub fn parse_one(s: &str) -> Result<(Self, &str), ParseError> {
        use ParseError::*;
        if let Some(mut s) = s.strip_prefix('[') {
            let mut list = vec![];
            if let Some(rest) = s.strip_prefix(']') {
                return Ok((Self::List(list), rest))
            }
            if s.is_empty() {
                return Err(UnterminatedList)
            }
            loop {
                let (val, rest) = Self::parse_one(s)?;
                list.push(val);
                let (c, rest) = {
                    let mut chars = rest.chars();
                    (chars.next(), chars.as_str())
                };
                match c {
                    Some(',') => (),
                    Some(']') => return Ok((Self::List(list), rest)),
                    Some(c) => return Err(InvalidSeperator(c)),
                    None => return Err(UnterminatedList),
                }
                s = rest;
            }
        } else {
            let end = s.find([',', ']']).unwrap_or(s.len());
            let (s, rest) = s.split_at(end);
            Ok((Self::Int(s.parse()?), rest))
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Int(a), Self::Int(b)) = (self, other) {
            a.cmp(b)
        } else {
            self.as_slice().cmp(other.as_slice())
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (val, rest) = Self::parse_one(s)?;
        if rest.is_empty() {
            Ok(val)
        } else {
            Err(Self::Err::DataAfterEnd)
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum ParseError {
    #[error("invalid integer value: {0}")]
    InvalidInt(#[from] ParseIntError),
    #[error("Invalid seperator: {0:?}")]
    InvalidSeperator(char),
    #[error("Missing terminating ] from")]
    UnterminatedList,
    #[error("Data after end")]
    DataAfterEnd,
}



type Data = Vec<(Packet, Packet)>;


#[aoc_generator(day13)]
pub fn generator(input: &str) -> Data {
    input.lines().collect::<Vec<&str>>().chunks(3).map(|c| {
        match c {
            [a, b, _] | [a, b] => (a.parse().unwrap(), b.parse().unwrap()),
            _ => unreachable!()
        }
    }).collect()
}

#[aoc(day13, part1)]
pub fn part1(inputs: &Data) -> usize {
    inputs.iter().enumerate().filter_map(|(n, (a, b))| {
        if a <= b {
            Some(n+1)
        } else {
            None
        }
    }).sum()
}


#[aoc(day13, part2)]
pub fn part2(inputs: &Data) -> usize {
    let mut all = inputs.into_iter().fold(vec![], |mut acc, (a, b)| {
        acc.push(a);
        acc.push(b);
        acc
    });
    let seps: [Packet; 2] = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    seps.iter().for_each(|s| all.push(s));
    all.sort();

    all.iter().enumerate().filter_map(|(n, p)| {
        if seps.contains(p) {
            Some(n+1)
        } else {
            None
        }
    }).product()
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 13);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 140);
    }
}
