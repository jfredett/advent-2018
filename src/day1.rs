use std::collections::HashMap;

pub type SignalChain = Vec<i64>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> SignalChain {
    input.lines().map(|l| {
        l.trim().parse().unwrap()
    }).collect()
}

#[aoc(day1, part1)]
pub fn solution(input: &SignalChain) -> i64 {
    let mut net = 0;
    for signal in input {
        net += signal
    }
    return net;
}

#[aoc(day1, part2)]
pub fn solution2(input: &SignalChain) -> i64 {
    let mut seen = HashMap::new();
    let mut curr = 0;

    let seen_count = seen.entry(curr).or_insert(0);
    *seen_count += 1;

    loop {
        for signal in input {
            curr += signal;

            let seen_count = seen.entry(curr).or_insert(0);

            if *seen_count == 1 {
                return curr;
            } else {
                *seen_count += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test1() {
        let input = vec![1, -1];
        assert_eq!(solution2(&input), 0);
    }

    #[test]
    fn part2_test2() {
        let input = vec![3, 3, 4, -2, -4];
        assert_eq!(solution2(&input), 10);
    }

    #[test]
    fn part2_test3() {
        let input = vec![-6,3,8,5,-6];
        assert_eq!(solution2(&input), 5);
    }
}
