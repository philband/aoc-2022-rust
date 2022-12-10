use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub enum Op {
    NOOP,
    ADDX(i32),
}

type Data = Vec<Op>;


fn format_crt(input: Vec<bool>) -> String {
    let s = (0..6).into_iter().map(|row| {
        (row*40..row*40+40).into_iter().map(|i| {
            match input[i] {
                true => "#",
                false => "."
            }
        }).collect::<String>()
    }).join("\n");
    format!("{}", s)
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<_>>();
            match parts[0] {
                "noop" => Op::NOOP,
                "addx" => Op::ADDX(parts[1].parse().unwrap()),
                _ => unreachable!()
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(inputs: &Data) -> i32 {
    let mut cycle = 1;
    let mut val = 1;
    let coi = vec![20,60,100,140,180,220];
    inputs.into_iter().fold_while(Vec::<i32>::new(), |mut acc, op| {
        match op {
            Op::NOOP => (),
            Op::ADDX(x) => {
                cycle += 1;
                if coi.iter().any(|&c| c == cycle) {
                    acc.push(cycle * val);
                }
                val += x;
            }
        }
        cycle += 1;
        if coi.iter().any(|&c| c == cycle) {
            acc.push(cycle * val);
        }
        return if cycle > coi[coi.len()-1] {
            Done(acc)
        } else {
            Continue(acc)
        }
    }).into_inner().iter().sum()
}


#[aoc(day10, part2)]
pub fn part2(inputs: &Data) -> String {
    let mut cycle: usize = 0;
    let mut val = 0;
    let mut sprite: Vec<i32> = vec![1,2,3];
    let mut screen = inputs.into_iter().fold_while(vec![false; 241], |mut acc, op| {
        cycle += 1;
        if sprite.iter().any(|&s| s % 40 == (cycle as i32) % 40) {
            acc[cycle] = true;
        }
        match op {
            Op::NOOP => (),
            Op::ADDX(x) => {
                val += x;
                sprite = vec![val, val+1, val+2];
                cycle += 1;
                if sprite.iter().any(|&s| s % 40 == (cycle as i32) % 40) {
                    acc[cycle] = true;
                }
            }
        }
        return if cycle >= 241 {
            Done(acc)
        } else {
            Continue(acc)
        }
    }).into_inner();
    screen[0] = true;
    format_crt(screen)
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";


    const RESULT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 13140);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), RESULT);
    }
}
