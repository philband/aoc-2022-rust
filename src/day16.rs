use std::collections::HashMap;
use ndarray::Array3;


#[aoc(day16, part1)]
pub fn part1(input: &str) -> u16 {
    let mut valves: Vec<(&str, u16, Vec<&str>)> = input.lines().fold(Vec::new(), |mut acc, line| {
        let (id, rate, _, r) = sscanf::sscanf!(line,
        "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}").unwrap();
        let routes = r.split(", ").collect::<Vec<_>>();
        acc.push((id, rate, routes));
        acc
    });

    valves.sort_by(|a, b| b.1.cmp(&a.1));
    // create index map
    let toidx = valves.iter().enumerate().map(|(i, v)| (v.0, i)).collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for v in valves.iter() {
        let i = toidx[v.0];
        flow[i] = v.1;
        for to in v.2.iter() {
            adj[i].push(toidx[to])
        }
    }
    let start = toidx["AA"];
    let mm = 1 << m;
    // m = valves with positive flow
    // [time left, current node, bitset of available valves]
    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o  = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1 , j, x)]);
                }
                opt[(t, i , x)] = o;
            }
        }
    }
    opt[(29, start, mm -1 )]
}


#[aoc(day16, part2)]
pub fn part2(input: &str) -> u16 {
    let mut valves: Vec<(&str, u16, Vec<&str>)> = input.lines().fold(Vec::new(), |mut acc, line| {
        let (id, rate, _, r) = sscanf::sscanf!(line,
        "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}").unwrap();
        let routes = r.split(", ").collect::<Vec<_>>();
        acc.push((id, rate, routes));
        acc
    });

    valves.sort_by(|a, b| b.1.cmp(&a.1));
    // create index map
    let toidx = valves.iter().enumerate().map(|(i, v)| (v.0, i)).collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for v in valves.iter() {
        let i = toidx[v.0];
        flow[i] = v.1;
        for to in v.2.iter() {
            adj[i].push(toidx[to])
        }
    }
    let start = toidx["AA"];
    let mm = 1 << m;
    // m = valves with positive flow
    // [time left, current node, bitset of available valves]
    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o  = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1 , j, x)]);
                }
                opt[(t, i , x)] = o;
            }
        }
    }
    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(opt[(25, start, x)] + opt[(25, start, y)])
    }
    best
}




#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 1651);
    }


    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 1707);
    }
}
