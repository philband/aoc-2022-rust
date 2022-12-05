use range_ext::intersect::*;

type Data = Vec<((i32, i32),(i32,i32))>;


#[aoc_generator(day4)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(",").into_iter();
            let mut p1 = parts.next().unwrap().split("-");
            let mut p2 = parts.next().unwrap().split("-");
            ((p1.next().unwrap().parse().unwrap(), p1.next().unwrap().parse().unwrap()), (p2.next().unwrap().parse().unwrap(), p2.next().unwrap().parse().unwrap()))
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(inputs: &Data) -> i32 {
    inputs.into_iter().map(|&((a, b),(x,y))| {
        match (a..b+1).intersect(&(x..y+1)) {
            Intersection::Same | Intersection::Within | Intersection::Over => 1,
            _ => 0
        }
    }).sum()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &Data) -> i32 {
    inputs.into_iter().map(|&((a, b),(x,y))| {
        match (a..b+1).intersect(&(x..y+1)) {
            Intersection::Bellow | Intersection::Above => 0,
            _ => 1
        }
    }).sum()
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
        assert_eq!(part1(&generator(&SAMPLE)), 2)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 4)
    }
}
