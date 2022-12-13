use aoc::*;

type Data = Vec<Vec<char>>;


#[aoc_generator(day12)]
pub fn generator(input: &str) -> Data {
    parse_grid(input.lines().collect::<Vec<_>>().as_slice())
}

#[aoc(day12, part1)]
pub fn part1(inputs: &Data) -> i64 {
    let start = inputs.points().find(|p| inputs.get_value(*p).unwrap() == 'S').unwrap();
    let dest = inputs.points().find(|p| inputs.get_value(*p).unwrap() == 'E').unwrap();
    astar_grid(
        inputs,
        |_p, _c| true,
        |_p1, c1, _p2, c2| {
            let height = if *c2 == 'E' { b'z' } else { *c2 as u8 };
            if *c1 as u8 +1 >= height || *c1 == 'S' {
                Some(1)
            } else {
                None
            }
        },
        start,
    dest
    ).unwrap().0
}

#[aoc(day12, part1, bfs)]
pub fn part1_bfs(inputs: &Data) -> usize {
    let start = inputs.points().find(|p| inputs.get_value(*p).unwrap() == 'S').unwrap();
    let dest = inputs.points().find(|p| inputs.get_value(*p).unwrap() == 'E').unwrap();
    bfs_grid(
        inputs,
        |_p1, c1, _p2, c2| {
            let height = if *c2 == 'E' { b'z' } else { *c2 as u8 };
            if *c1 as u8 +1 >= height || *c1 == 'S' {
                true
            } else {
                false
            }
        },
        start,
        dest
    ).unwrap().len()
}


#[aoc(day12, part2)]
pub fn part2(inputs: &Data) -> i64 {
    let starts = inputs.points().filter(|p| inputs.get_value(*p).unwrap() == 'a').collect::<Vec<_>>();
    let dest = inputs.points().find(|p| inputs.get_value(*p).unwrap() == 'E').unwrap();
    starts.iter().filter_map(|start| {
        if let Some(p) = astar_grid(
            inputs,
            |_p, _c| true,
            |_p1, c1, _p2, c2| {
                let height = if *c2 == 'E' { b'z' } else { *c2 as u8 };
                if *c1 as u8 +1 >= height || *c1 == 'S' {
                    Some(1)
                } else {
                    None
                }
            },
            *start,
            dest
        ) {
            Some(p.0)
        } else {
            None
        }
    }).min().unwrap()
}

#[aoc(day12, part2, bfs)]
pub fn part2_bfs(inputs: &Data) -> usize {
    let starts = inputs.points().filter(|p| inputs.get_value(*p).unwrap() == 'a').collect::<Vec<_>>();
    let dest = inputs.points().find(|p| inputs.get_value(*p).unwrap() == 'E').unwrap();
    starts.iter().filter_map(|&start| {
        if let Some(p) = bfs_grid(
            inputs,
            |_p1, c1, _p2, c2| {
                let height = if *c2 == 'E' { b'z' } else { *c2 as u8 };
                if *c1 as u8 +1 >= height || *c1 == 'S' {
                    true
                } else {
                    false
                }
            },
            start,
            dest
        ) {
            Some(p.len())
        } else {
            None
        }
    }).min().unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 31);
    }

    #[test]
    pub fn test1_bfs() {
        assert_eq!(part1_bfs(&generator(&SAMPLE)), 31);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 29);
    }

    #[test]
    pub fn test2_bfs() {
        assert_eq!(part2_bfs(&generator(&SAMPLE)), 29);
    }
}
