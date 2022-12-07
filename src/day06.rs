 use std::collections::HashSet;
 use itertools::Itertools;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &String) -> usize {
    solve(inputs, 4)
}

 #[aoc(day6, part1, noset)]
 pub fn part1_noset(inputs: &String) -> usize {
     solve2(inputs, 4)
 }

#[aoc(day6, part2)]
pub fn part2(inputs: &String) -> usize {
    solve(inputs, 14)
}

 #[aoc(day6, part2, noset)]
 pub fn part2_noset(inputs: &String) -> usize {
     solve2(inputs, 14)
 }

 pub fn solve(inputs: &String, len: usize) -> usize {
     let charin = inputs.chars().collect_vec();
     charin.windows(len).enumerate().find(|(_, window)| {
         let mut set = HashSet::<char>::new();
         let mut found = true;
         window.iter().for_each(|&c| {
             if set.contains(&c) {
                 found = false;
             } else {
                 set.insert(c);
             }
         });
         return found
     }).unwrap().0 + len
 }

 pub fn solve2(inputs: &String, len: usize) -> usize {
     let charin = inputs.chars().collect_vec();
     charin.windows(len).position(|arr| arr.iter().all_unique()).unwrap() + len
 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test1() {
        assert_eq!(part1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(part1(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(part1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(part1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);

        assert_eq!(part1_noset(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(part1_noset(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(part1_noset(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(part1_noset(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
    #[test]
    pub fn test2() {
        assert_eq!(part2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(part2(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(part2(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(part2(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(part2(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);

        assert_eq!(part2_noset(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(part2_noset(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(part2_noset(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(part2_noset(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(part2_noset(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);
    }
}
