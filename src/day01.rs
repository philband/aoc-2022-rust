type Data = Vec<i32>;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Data {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &Data) -> usize {
    0
}

#[aoc(day1, part2)]
pub fn part2(inputs: &Data) -> usize {
    0
}
#[cfg(test)]
mod tests {
    //use super::*;

    const _SAMPLE: [i32; 10] = [199,200,208,210,200,207,240,269,260,263];

    #[test]
    pub fn test1() {
        //assert_eq!(part1(&SAMPLE), 7);
        //assert_eq!(part1_golf1(&SAMPLE), 7);
    }

    #[test]
    pub fn test2() {
        //assert_eq!(part2(&SAMPLE), 5);
        //assert_eq!(part2_golf1(&SAMPLE), 5);
    }
}
