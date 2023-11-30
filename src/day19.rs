use itertools::Itertools;
use sscanf::sscanf;
use crate::day19::Material::{Clay, Geode, Obsidian, Ore};
use rayon::prelude::*;


#[derive(Copy, Clone)]
pub enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

type RecipePart = (u32, Material);

pub struct Blueprint {
    id: u32,
    recipes: [Vec<RecipePart>; 4]
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct State {
    time_remaining: u32,
    robots: [u32; 4],
    materials: [u32; 4],
}

impl State {
    fn can_build_robot(&self, robot_type: usize, bp: &Blueprint, max_materials: &[u32]) -> bool {
        let r = &bp.recipes[robot_type];
        self.robots[robot_type] < max_materials[robot_type] && r.iter().all(|&(n, m)| self.materials[m as usize] >= n)
    }

    fn build_robot(&mut self, robot_type: usize, bp: &Blueprint) {
        self.robots[robot_type] += 1;
        for &(n, m) in &bp.recipes[robot_type] {
            self.materials[m as usize] -= n;
        }
    }

    fn destroy_robot(&mut self, robot_type: usize, bp: &Blueprint) {
        self.robots[robot_type] -= 1;
        for &(n, m) in &bp.recipes[robot_type] {
            self.materials[m as usize] += n;
        }
    }
}

type Data = Vec<Blueprint>;

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Data {
    input.lines().map(|line| {
        let (id, ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs) = sscanf!(
            line,
            "Blueprint {u32}: Each ore robot costs {u32} ore. Each clay robot costs {u32} ore. Each obsidian robot costs {u32} ore and {u32} clay. Each geode robot costs {u32} ore and {u32} obsidian.").unwrap();
        Blueprint{
            id,
            recipes: [
                vec![(ore_ore, Ore)],
                vec![(clay_ore, Ore)],
                vec![(obs_ore, Ore), (obs_clay, Clay)],
                vec![(geo_obs, Obsidian), (geo_ore, Ore)]
            ]
        }
    }).collect()
}

pub fn get_blueprint_score(bp: &Blueprint, time_remaining: u32) -> u32 {
    let state = State { time_remaining, robots: [1, 0, 0, 0], materials: [0, 0, 0, 0]};
    run_blueprint_production(&state, bp, &max_mat(bp), None, 0)
}

pub fn run_blueprint_production(
    state: &State,
    bp: &Blueprint,
    max_materials: &[u32],
    prev_skipped: Option<&Vec<usize>>,
    best_so_far: u32
) -> u32 {
    if state.time_remaining == 1 {
        return state.materials[Geode as usize] + state.robots[Geode as usize];
    }

    if production_estimation(state, Geode) < best_so_far {
        return 0;
    }

    if production_estimation(state, Obsidian) < max_materials[Obsidian as usize] {
        return state.materials[Geode as usize] + state.robots[Geode as usize] * state.time_remaining;
    }

    let mut new_state = *state;
    new_state.time_remaining -= 1;
    (0..4).for_each(|m| new_state.materials[m] += new_state.robots[m]);

    if state.can_build_robot(Geode as usize, bp, max_materials) {
        new_state.build_robot(Geode as usize, bp);
        return run_blueprint_production(&new_state, bp, max_materials, None, best_so_far);
    }

    let available_robots = (0..3).filter(|r| state.can_build_robot(*r, bp, max_materials)).collect_vec();
    let mut best = best_so_far;
    for &r in &available_robots {
        if prev_skipped.map(|ps| ps.contains(&r)).unwrap_or(false) {
            continue
        }

        new_state.build_robot(r, bp);
        let score = run_blueprint_production(&new_state, bp, max_materials, None, best);
        best = u32::max(best, score);
        new_state.destroy_robot(r, bp);
    }
    let score = run_blueprint_production(&new_state, bp, max_materials, Some(&available_robots), best);
    u32::max(best, score)
}

pub fn production_estimation(state: &State, material: Material) -> u32 {
    let m = material as usize;
    let i = state.time_remaining;
    state.materials[m]
    + state.robots[m] * i
    + i * (i-1) / 2
}

pub fn max_mat(bp: &Blueprint) -> [u32; 4] {
    let mut max = [0, 0, 0, u32::MAX];
    for r in &bp.recipes {
        for &(n, m) in r {
            let i = m as usize;
            max[i] = u32::max(max[i], n)
        }
    }
    max
}

#[aoc(day19, part1)]
pub fn part1(inputs: &Data) -> u32 {
    inputs.par_iter().map(|bp| bp.id * get_blueprint_score(bp, 24)).sum()
}


#[aoc(day19, part2)]
pub fn part2(inputs: &Data) -> u32 {
    inputs.par_iter().take(3).map(|bp| get_blueprint_score(bp, 32)).product()
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 33);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 0);
    }
}
