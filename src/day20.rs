use std::collections::{VecDeque};



type Data = Vec<i64>;

const DECRYPTION_KEY: i64 = 811589153;


#[aoc_generator(day20)]
pub fn generator(input: &str) -> Data {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn decrypt(data: &Data, rounds: usize, key: i64) -> i64 {
    let len = data.len();
    let mut indicies = (0..len).collect::<VecDeque<_>>();
    for _round in 0..rounds {
        for i in 0..len {
            let p = indicies.iter().position(|&p| p == i).unwrap();
            let num = data[i] * key;
            indicies.remove(p);
            let new_index = (p as i64 + num).rem_euclid(len as i64 - 1);
            indicies.insert(new_index as usize, i);
        }
    }

    let numbers = indicies.iter().map(|&i| data[i]).collect::<Vec<_>>();
    let zero = numbers.iter().position(|&n| n == 0).unwrap();

    [1000,2000,3000].iter().map(|i| numbers[(zero + i) % len] * key).sum()
}


#[aoc(day20, part1)]
pub fn part1(inputs: &Data) -> i64 {
    decrypt(inputs, 1, 1)
}


#[aoc(day20, part2)]
pub fn part2(inputs: &Data) -> i64 {
    decrypt(inputs, 10, DECRYPTION_KEY)
}



#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1
2
-3
3
-2
0
4";


    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 3);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 1623178306);
    }
}
