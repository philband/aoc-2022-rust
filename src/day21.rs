use std::collections::HashMap;
use crate::day21::MonkeyAction::{Minus, Plus, Mul, Div, Val, Eq};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MonkeyAction {
    Plus(i64, i64),
    Minus(i64, i64),
    Mul(i64, i64),
    Div(i64, i64),
    Eq(i64, i64),
    Val(i64)
}


type Data = HashMap<i64, MonkeyAction>;


#[aoc_generator(day21)]
pub fn generator(input: &str) -> Data {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let chars = line.chars().collect::<Vec<_>>();
        let cur = chars[0..4].iter().fold(0, |acc, &c| acc * 256 + (c as i64 - 'a' as i64));
        if chars[6].is_numeric() {
            acc.insert(cur, Val(chars[6..].iter().collect::<String>().parse().unwrap()));
        } else {
            let a = chars[6..10].iter().fold(0, |acc, &c| acc * 256 + (c as i64 - 'a' as i64));
            let b = chars[13..17].iter().fold(0, |acc, &c| acc * 256 + (c as i64 - 'a' as i64));
            let op = match chars[11] {
                '+' => Plus(a, b),
                '-' => Minus(a, b),
                '*' => Mul(a, b),
                '/' => Div(a, b),
                _ => unreachable!()
            };
            acc.insert(cur, op);
        }
        acc
    })
}

pub fn contains_monkey(map: &HashMap<i64, MonkeyAction>, current: &i64, needle: &i64) -> bool {
    if current == needle {
        return true;
    }
    return match map.get(current).unwrap() {
        Plus(a, b) | Minus(a, b) | Mul(a, b) | Div(a, b) => contains_monkey(map, a, needle) || contains_monkey(map, b, needle),
        _ => false
    }
}


impl MonkeyAction {
    fn eval(&self, map: &HashMap<i64, MonkeyAction>) -> i64 {
        match self {
            Val(n) => *n,
            Plus(a, b) => map.get(a).unwrap().eval(map) + map.get(b).unwrap().eval(map),
            Minus(a, b) => map.get(a).unwrap().eval(map) - map.get(b).unwrap().eval(map),
            Mul(a, b) => map.get(a).unwrap().eval(map) * map.get(b).unwrap().eval(map),
            Div(a, b) => map.get(a).unwrap().eval(map) / map.get(b).unwrap().eval(map),
            _ => 0,
        }
    }

    fn inv_right(&self, map: &HashMap<i64, MonkeyAction>, v: &i64) -> i64 {
        match self {
            Plus(a, _) => *v - map.get(a).unwrap().eval(map),
            Minus(a, _) => map.get(a).unwrap().eval(map) - *v,
            Mul(a, _) => *v / map.get(a).unwrap().eval(map),
            Div(a, _) => map.get(a).unwrap().eval(map) / *v,
            _ => 0
        }
    }
    fn inv_left(&self, map: &HashMap<i64, MonkeyAction>, v: &i64) -> i64 {
        match self {
            Plus(_, b) => *v - map.get(b).unwrap().eval(map),
            Minus(_, b) => *v + map.get(b).unwrap().eval(map),
            Mul(_, b) => *v / map.get(b).unwrap().eval(map),
            Div(_, b) => *v * map.get(b).unwrap().eval(map),
            _ => 0
        }
    }
}

pub fn solve2(map: &HashMap<i64, MonkeyAction>, humn: &i64, current: &i64, target: &i64) -> i64 {
    if current == humn {
        return *target;
    }
    if let Some((a, b)) = match map.get(current).unwrap() {
        Plus(a, b) | Minus(a, b) | Mul(a, b) | Div(a, b) => Some((a, b)),
        _ => None
    } {
        let op = map.get(current).unwrap();
        return if contains_monkey(map, b, humn) {
            solve2(map, humn, b, &op.inv_right(map, target))
        } else {
            solve2(map, humn, a, &op.inv_left(map, target))
        }
    } else if let Val(n) = map.get(current).unwrap() {
        return *n;
    }
    unreachable!()
}



#[aoc(day21, part1)]
pub fn part1(inputs: &Data) -> i64 {
    let root = ['r', 'o', 'o', 't'].iter().fold(0, |acc, &c| acc * 256 + (c as i64 - 'a' as i64));
    inputs.get(&root).unwrap().eval(inputs)
}


#[aoc(day21, part2)]
pub fn part2(inputs: &Data) -> i64 {
    let mut inputs = inputs.clone();
    let root = ['r', 'o', 'o', 't'].iter().fold(0, |acc, &c| acc * 256 + (c as i64 - 'a' as i64));
    let humn = ['h', 'u', 'm', 'n'].iter().fold(0, |acc, &c| acc * 256 + (c as i64 - 'a' as i64));
    let mut target = 0;
    let mut to_solve = 0;
    if let Plus(a, b) = inputs.get(&root).unwrap().clone() {
        let r = inputs.get_mut(&root).unwrap();
        *r = Eq(a, b);
        if contains_monkey(&inputs, &a, &humn) {
            target = inputs.get(&b).unwrap().eval(&inputs);
            to_solve = a;
        } else {
            target = inputs.get(&a).unwrap().eval(&inputs);
            to_solve = b;
        };
    }

    solve2(&inputs, &humn, &to_solve, &target)
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 152);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 301);
    }
}
