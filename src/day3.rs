use std::str;
use std::fmt;
use std::collections::HashMap;

fn parse_u8(input: &[u8]) -> u8 {
    let s = str::from_utf8(input).ok().unwrap();
    u8::from_str_radix(s, 10).ok().unwrap()
}

fn parse_u16(input: &[u8]) -> u16 {
    let s = str::from_utf8(input).ok().unwrap();
    u16::from_str_radix(s, 10).ok().unwrap()
}

// <w>x<h>
named!(dimensions<(u8, u8)>,
    do_parse!(
       x : separated_pair!(take_while!(nom::is_digit), char!('x'), nom::rest) >>
       ( (parse_u8(x.0), parse_u8(x.1)) )
    )
);

// <x>,<y>
named!(coordinates<(u16,u16)>,
    do_parse!(
        x: take_until_and_consume!(",") >>
        y: take_until_and_consume!(":") >>
        ( (parse_u16(x), parse_u16(y)) )
    )
);

// <id>
named!(id<u16>,
   do_parse!(
           tag!("#")                    >>
       id: take_until_and_consume!(" ") >>
       ( parse_u16(id) )
   )
);

// #<id> @ <coordinates>: <dimensions>
named!(claim<Claim>,
   do_parse!(
       id:     id          >>
               tag!("@ ")  >>
       coords: coordinates >>
               tag!(" ")   >>
       dims:   dimensions  >>
       (Claim { id: id, x_pos: coords.0, y_pos: coords.1, width: dims.0, height: dims.1 })
   )
);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Claim {
    id: u16,
    x_pos: u16,
    y_pos: u16,
    width: u8,
    height: u8
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.x_pos, self.y_pos, self.width, self.height)
    }

}

impl Claim {
    pub fn top_left_corner(&self) -> (u16,u16) {
        (self.x_pos, self.y_pos)
    }

    pub fn top_right_corner(&self) -> (u16,u16) {
        (self.x_pos + self.width as u16, self.y_pos)
    }

    pub fn bottom_left_corner(&self) -> (u16,u16) {
        (self.x_pos, self.y_pos + self.height as u16)
    }

    pub fn bottom_right_corner(&self) -> (u16,u16) {
        (self.x_pos + self.width as u16, self.y_pos + self.height as u16)
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let mut result = vec![];
    for line in input.lines() {
        let c = claim(line.as_bytes());

        match c {
            Ok((_, cl)) => result.push(cl),
            e => { println!("{:?}", e); panic!("Parse error"); }
        }
    }
    result
}

fn conflict_map(input: &Vec<Claim>) -> HashMap<(u16,u16), u64> {
    let mut seen = HashMap::new();

    for claim in input {
        for i in claim.x_pos..(claim.x_pos + claim.width as u16) {
            for j in claim.y_pos..(claim.y_pos + claim.height as u16) {
                let seen_count = seen.entry((i,j)).or_insert(0);
                *seen_count += 1;
            }
        }
    }

    return seen;
}

#[aoc(day3, part1)]
pub fn solution(input: &Vec<Claim>) -> u64 {
    let mut count = 0;
    for (_,v) in conflict_map(input) {
        if v > 1 { count += 1; }
    }

    return count
}

#[aoc(day3, part2)]
pub fn solution2(input: &Vec<Claim>) -> Claim {
    let mut map = conflict_map(input);

    'start: for claim in input {
        let mut found_conflict;
        found_conflict = false;
        for i in claim.x_pos..(claim.x_pos + claim.width as u16) {
            for j in claim.y_pos..(claim.y_pos + claim.height as u16) {
                let entry = map.entry((i,j)).or_insert(0);
                found_conflict |= *entry > 1;
                if found_conflict { continue 'start }
            }
        }
        // if we get here, we never found a conflict, so this is the right thing
        return claim.clone();
    }
    panic!("Cannot be reached");
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solutions {
        use super::*;
        mod part2 {
            use super::*;

            #[test]
            fn single_claim() {
                let input = vec![
                    Claim { id: 3, x_pos: 5, y_pos: 5, width: 2, height: 2 },
                ];
                let sol = solution2(&input);
                assert_eq!(sol.id, 3);
            }

            #[test]
            fn multiple_overlapping_claims() {
                let input = vec![
                    Claim { id: 1, x_pos: 5, y_pos: 5, width: 2, height: 3 },
                    Claim { id: 2, x_pos: 5, y_pos: 5, width: 3, height: 2 },
                    Claim { id: 3, x_pos: 0, y_pos: 0, width: 2, height: 2 },
                ];

                let sol = solution2(&input);
                assert_eq!(sol.id, 3);
            }
        }

    }
    mod generator {
        use super::*;

        #[test]
        fn claim_generator() {
            let claims = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
            let generated = input_generator(claims);

            let expected = vec![
                Claim { id: 1, x_pos: 1, y_pos: 3, width: 4, height: 4 },
                Claim { id: 2, x_pos: 3, y_pos: 1, width: 4, height: 4 },
                Claim { id: 3, x_pos: 5, y_pos: 5, width: 2, height: 2 },
            ];

            assert_eq!(generated, expected);
        }


    }

    mod parser {
        use super::*;

        #[test]
        fn claim_parser() {
            let input = b"#1 @ 1,3: 4x4";
            let expected = Claim { id: 1, x_pos: 1, y_pos: 3, width: 4, height: 4 };
            assert_eq!(claim(input), Ok((&[][..], expected)));
        }


        #[test]
        fn id_parser() {
            let input = b"#1 ";
            let expected = 1;
            assert_eq!(id(input), Ok((&[][..], expected)));

        }

        #[test]
        fn coordinates_parser() {
            let input = b"1,3:";
            let expected = (1,3);

            assert_eq!(coordinates(input), Ok((&[][..], expected)));
        }

        #[test]
        fn dimensions_parser() {
            let input = b"4x4";
            let expected = (4,4);

            assert_eq!(dimensions(input), Ok((&[][..], expected)));
        }
    }
}
