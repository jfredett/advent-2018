use std::collections::HashMap;

pub type BoxID = HashMap<char, u64>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<BoxID> {
    input.lines().map(|l| {
        let mut box_id = HashMap::new();
        for c in l.trim().chars() {
            let letter_count = box_id.entry(c).or_insert(0);
            *letter_count += 1
        }
        box_id
    }).collect()
}

pub fn signature(boxid: BoxID) -> (bool,bool) {
    let mut two_flag = false;
    let mut three_flag = false;

    for (_,v) in boxid {
        two_flag   |= v == 2;
        three_flag |= v == 3;
    }

    return (two_flag, three_flag);
}

#[aoc(day2, part1)]
pub fn solution(input: &Vec<BoxID>) -> i64 {
    let mut two_boxes = 0;
    let mut three_boxes = 0;

    for b in input {
        match signature(b.clone()) {
            (true, true)   => { two_boxes +=1; three_boxes +=1 },
            (true, false)  => { two_boxes +=1; },
            (false, true)  => { three_boxes +=1; },
            (false, false) => ()
        }
    }
    return two_boxes * three_boxes;
}

#[aoc(day2, part2)]
pub fn solution2(input: &Vec<BoxID>) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_test1() {
        let example = "abcdef";
        let gen_input = input_generator(example);
        let input = gen_input.get(0).unwrap();

        assert_eq!(input.get(&'a'), Some(&1u64));
    }

    #[test]
    fn generator_test2() {
        let example = "bababc";
        let gen_input = input_generator(example);
        let input = gen_input.get(0).unwrap();

        assert_eq!(input.get(&'a'), Some(&2u64));
        assert_eq!(input.get(&'b'), Some(&3u64));
    }

    #[test]
    fn example_checksum() {
        let examples = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        let gen_input = input_generator(examples);

        assert_eq!(solution(&gen_input), 12);
    }

}
