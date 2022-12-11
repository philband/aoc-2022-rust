use std::collections::VecDeque;
use itermore::IterSorted;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Op {
    Add(i64),
    Mul(i64),
    MulSelf,
}

impl Op {
    fn apply(&self, val: i64) -> i64 {
        match self {
            Op::Add(x) => val + x,
            Op::Mul(x) => val * x,
            Op::MulSelf => val * val,
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test_div: i64,
    target_true: usize,
    target_false: usize,
    inspection_count: i64,
}

impl Monkey {
    fn throw_target(&self, item: i64) -> usize {
        match item % self.test_div {
            0 => self.target_true,
            _ => self.target_false,
        }
    }
}

type Data = Vec<Monkey>;


#[aoc_generator(day11)]
pub fn generator(input: &str) -> Data {
    let mut data = Vec::<Monkey>::new();

    for lines in input.lines().collect::<Vec<&str>>().chunks(7) {

        let items = lines[1].trim().trim_start_matches("Starting items: ").split(", ").map(|item| item.parse::<_>().unwrap()).collect::<VecDeque<_>>();
        let op_line = lines[2].trim().trim_start_matches("Operation: new = old ");
        let op = match op_line.chars().nth(0).unwrap() {
            '*' => match &op_line[2..] {
                "old" => Op::MulSelf,
                _ => Op::Mul(op_line[2..].parse().unwrap())
            },
            '+' => Op::Add(op_line[2..].parse().unwrap()),
            _ => unreachable!()
        };
        let test_div = lines[3].trim().trim_start_matches("Test: divisible by ").parse::<_>().unwrap();
        let target_true = lines[4].trim().trim_start_matches("If true: throw to monkey ").parse::<usize>().unwrap();
        let target_false = lines[5].trim().trim_start_matches("If false: throw to monkey ").parse::<usize>().unwrap();
        data.push(Monkey{
            items,
            op,
            test_div,
            target_true,
            target_false,
            inspection_count: 0,
        })
    }
    data
}

#[aoc(day11, part1)]
pub fn part1(inputs: &Data) -> i64 {
    run(inputs, 20, true)
}


#[aoc(day11, part2)]
pub fn part2(inputs: &Data) -> i64 {
    run(inputs, 10000, false)
}

pub fn run(inputs: &Data, rounds: usize, div3: bool) -> i64 {
    let gcd = inputs.iter().map(|m| m.test_div).product::<i64>();
    let mut monkeys = inputs.clone();
    for _i in 0..rounds {
        for n in 0..monkeys.len() {
            while ! monkeys[n].items.is_empty() {
                let mut item = monkeys[n].items.pop_front().unwrap();
                if div3 {
                    item = (monkeys[n].op.apply(item) / 3) % gcd;
                } else {
                    item = monkeys[n].op.apply(item) % gcd;
                }
                let to = monkeys[n].throw_target(item);
                monkeys[n].inspection_count += 1;
                monkeys[to].items.push_back(item);
            }
        }
    }
    monkeys.iter().map(|m| m.inspection_count).sorted().rev().take(2).product()
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 10605);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 2713310158);
    }
}
