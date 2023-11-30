use aoc::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Elem {
    Up(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
    Right(usize, usize),
    Wall(usize, usize),
    Open(usize, usize)
}

impl Elem {
    fn from_char(c: char, x: usize, y: usize) -> Option<Self> {
        use Elem::*;
        match c {
            '^' => Some(Up(y, x)),
            'v' => Some(Down(y, x)),
            '<' => Some(Left(y, x)),
            '>' => Some(Right(y, x)),
            '#' => Some(Wall(y, x)),
            '.' => Some(Open(y, x)),
            _ => None
        }
    }

    fn step(&self, xmax: usize, ymax: usize) -> Self {
        use Elem::*;
        match self {
            &Up(1, x) => Up(ymax - 1, x),
            &Up(y, x) => Up(y - 1 , x),
            &Down(y, x) if y == ymax - 1 => Down(1, x),
            &Down(y, x) => Down(y + 1, x),
            &Left(y, 1) => Left(y, xmax - 1),
            &Left(y, x) => Left(y, x - 1),
            &Right(y, x) if x == xmax - 1 => Right(y, 1),
            &Right(y, x) => Right(y, x + 1),
            _ => *self,
        }
    }
}

type Data = Vec<Elem>;


#[aoc_generator(day24)]
pub fn generator(input: &str) -> Data {
    input.lines().enumerate().fold(Vec::new(), |map, (y, line)| {
        line.chars().enumerate().fold(map, |mut map, (x, c)| {
            if let Some(b) = Elem::from_char(c, x, y) {
                map.push(b);
            }
            map
        })
    })
}



#[aoc(day24, part1)]
pub fn part1(inputs: &Data) -> i64 {
    0
}


#[aoc(day24, part2)]
pub fn part2(inputs: &Data) -> i32 {
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 18);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 0);
    }
}
