use std::collections::{BinaryHeap, HashMap, HashSet};
use aoc::*;
use ndarray::Array3;


type Data = Vec<(String, u16, Vec<String>)>;

pub fn parse_valve_id(id: &str) -> usize {
    let mut chars = id.chars().map(|c| c as usize - 'A' as usize);
    chars.next().unwrap() + chars.next().unwrap() * 26
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct Valve {
    id: usize,
    rate: i64,
    routes: Vec<usize>,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Copy)]
pub struct State {
    pos: usize,
    opened: [usize; 64],
}

#[derive(Debug)]
pub struct ValveSimulator {
    valves: HashMap<usize, Valve>,
}

impl ValveSimulator {
    pub fn get_possible_targets(&self, pos: usize) -> Vec<&Valve> {
        self.valves.get(&pos).unwrap().routes.iter().map(|r| self.valves.get(r).unwrap()).collect()
    }
}



#[aoc_generator(day16)]
pub fn generator(input: &str) -> Data {
    let mut valves: Vec<(&str, u16, Vec<&str>)> = input.lines().fold(Vec::new(), |mut acc, line| {
        let (id, rate, _, r) = sscanf::sscanf!(line,
        "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/}, {str}").unwrap();
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



    input.lines().fold(HashMap::new(), |mut acc, line| {
        let parts = line.trim().split("; ").collect::<Vec<&str>>();

        let (id, rate) = match parts[0].trim().split([' ', '=']).collect::<Vec<&str>>().as_slice() {
            ["Valve", v, "has", "flow", "rate", n] => (parse_valve_id(v), n.parse().unwrap()),
            _ => unreachable!()
        };

        let routes = parts[1]
            .split("valve").nth(1).unwrap().trim_start_matches("s")
            .trim().split(", ")
            .map(|to| parse_valve_id(to))
            .collect();

        acc.insert(id, Valve {
            id,
            rate,
            routes,
        });
        acc

    })
}

fn simulate(pos: usize, vs: &ValveSimulator, minute: i64, time_cap: i64) -> i64 {
    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::<State>::new();
    let mut best = 0;
    frontier.push(
    (
            0,
            0,
            0,
            State {
                pos,
                opened: [0; 64]
            },
            minute,
        ),
    );
    while let Some((e_score, score, flow, state, minute)) = frontier.pop() {
        if e_score < best {
            break;
        }
        if minute == time_cap {
            if score > best {
                best = score;
            }
            continue;
        }
        // check targets
        for t in vs.get_possible_targets(state.pos) {

            for x in 0..2 {
                // 2 runs due to possible time spent opening
                if x == 1 && vs.valves.get(&state.pos).unwrap().rate == 0 {
                    // second step & flow rate of valve == 0 => skip
                    continue
                }

                let mut o = state.opened;
                let mut tl = minute;

                let mut sc = 0;
                let mut nf = flow;
                if x == 1 && ! o.contains(&state.pos) {
                    *o.iter_mut().find(|x| **x == 0).unwrap() = state.pos;
                    let f = vs.valves.get(&state.pos).unwrap().rate;
                    sc += (time_cap - tl) * f;
                    nf += f;
                    tl += 1;
                }
                if tl > time_cap {
                    continue;
                }

                let new_score = score + sc;
                let ns = State { pos: t.id, opened: o};
                if visited.insert(ns.clone()) {
                    let next = (new_score + nf, new_score, nf, ns, tl +1);
                    frontier.push(next);
                }
            }
        }
    }
    best
}

#[aoc(day16, part1)]
pub fn part1(inputs: &Data) -> i64 {
    simulate(parse_valve_id("AA"), &ValveSimulator{valves: inputs.clone()}, 1, 30)
}


#[aoc(day16, part2)]
pub fn part2(inputs: &Data) -> i64 {
    0
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
        assert_eq!(part1(&generator(&SAMPLE)), 1651);
    }


    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 0);
    }
}
