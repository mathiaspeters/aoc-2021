use std::str::Chars;

pub fn day18() {
    println!("Result 18-1: {}", part1());
    println!("Result 18-2: {}", part2());
}

pub fn part1() -> usize {
    let snail_numbers = raw_input()
        .lines()
        .map(|s| SnailNumber::new(s))
        .collect::<Vec<_>>();
    let result = SnailNumber::add_multiple(snail_numbers);
    result.magnitude()
}

pub fn part2() -> usize {
    let snail_numbers = raw_input()
        .lines()
        .map(|s| SnailNumber::new(s))
        .collect::<Vec<_>>();
    let mut max_magnitude = 0;
    for i in 0..snail_numbers.len() {
        for j in 0..snail_numbers.len() {
            let magnitude =
                SnailNumber::add(snail_numbers[i].clone(), snail_numbers[j].clone()).magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    max_magnitude
}

#[derive(Clone, Debug, PartialEq)]
enum SnailNumber {
    Literal(usize),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl SnailNumber {
    pub fn new(s: &str) -> Self {
        let s = s.replace(",", "").replace("]", "");
        let mut c = s.chars();
        c.next().unwrap();
        let sn1 = Self::parse(&mut c);
        let sn2 = Self::parse(&mut c);
        Self::Pair(Box::new(sn1), Box::new(sn2))
    }

    fn parse(c: &mut Chars) -> SnailNumber {
        let ch = c.next().unwrap();
        match super::util::char_to_u8(ch) {
            Some(num) => Self::Literal(num as usize),
            None => {
                let sn1 = Self::parse(c);
                let sn2 = Self::parse(c);
                Self::Pair(Box::new(sn1), Box::new(sn2))
            }
        }
    }

    pub fn add_multiple(numbers: Vec<SnailNumber>) -> SnailNumber {
        let mut result = numbers[0].clone();
        numbers.into_iter().skip(1).for_each(|sn| {
            result = SnailNumber::add(result.clone(), sn);
        });
        result
    }

    pub fn add(sn1: SnailNumber, sn2: SnailNumber) -> SnailNumber {
        let mut num = SnailNumber::Pair(Box::new(sn1), Box::new(sn2));
        num.reduce();
        num
    }

    pub fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            } else if self.split() {
                continue;
            } else {
                break;
            }
        }
    }

    pub fn explode(&mut self) -> bool {
        match self {
            SnailNumber::Literal(_) => false,
            SnailNumber::Pair(sn1, sn2) => {
                if let Some((_, next)) = sn1.explode_rec(1) {
                    if next > 0 {
                        sn2.add_to_first_literal(next);
                    }
                    true
                } else if let Some((prev, _)) = sn2.explode_rec(1) {
                    if prev > 0 {
                        sn1.add_to_last_literal(prev);
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn add_to_first_literal(&mut self, value: usize) -> bool {
        match self {
            SnailNumber::Literal(num) => {
                *num += value;
                return true;
            }
            SnailNumber::Pair(sn1, sn2) => {
                if sn1.add_to_first_literal(value) {
                    return true;
                } else if sn2.add_to_first_literal(value) {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    fn add_to_last_literal(&mut self, value: usize) -> bool {
        match self {
            SnailNumber::Literal(num) => {
                *num += value;
                return true;
            }
            SnailNumber::Pair(sn1, sn2) => {
                if sn2.add_to_last_literal(value) {
                    return true;
                } else if sn1.add_to_last_literal(value) {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    pub fn explode_rec(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            SnailNumber::Literal(_) => None,
            SnailNumber::Pair(sn1, sn2) => {
                if depth < 4 {
                    match sn1.explode_rec(depth + 1) {
                        Some((prev, next)) => {
                            if sn2.add_to_first_literal(next) {
                                Some((prev, 0))
                            } else {
                                Some((prev, next))
                            }
                        }
                        None => match sn2.explode_rec(depth + 1) {
                            Some((prev, next)) => {
                                if sn1.add_to_last_literal(prev) {
                                    Some((0, next))
                                } else {
                                    Some((prev, next))
                                }
                            }
                            None => None,
                        },
                    }
                } else {
                    match (sn1.as_mut(), sn2.as_mut()) {
                        (Self::Literal(p), Self::Literal(n)) => {
                            let ret = Some((*p, *n));
                            std::mem::swap(self, &mut Self::Literal(0));
                            ret
                        }
                        (Self::Pair(_, _), _) => sn1.explode_rec(depth + 1),
                        (_, Self::Pair(_, _)) => sn2.explode_rec(depth + 1),
                    }
                }
            }
        }
    }

    pub fn split(&mut self) -> bool {
        match self {
            SnailNumber::Literal(num) => {
                if *num > 9 {
                    let mut new_num = SnailNumber::Pair(
                        Box::new(SnailNumber::Literal(*num / 2)),
                        Box::new(SnailNumber::Literal((*num / 2) + (*num & 1))),
                    );
                    std::mem::swap(self, &mut new_num);
                    return true;
                }
            }
            SnailNumber::Pair(sn1, sn2) => {
                if sn1.split() || sn2.split() {
                    return true;
                }
            }
        }
        false
    }

    pub fn magnitude(&self) -> usize {
        match self {
            SnailNumber::Literal(num) => *num as usize,
            SnailNumber::Pair(sn1, sn2) => (3 * sn1.magnitude()) + (2 * sn2.magnitude()),
        }
    }

    /*pub fn serialize(&self) -> String {
        match self {
            SnailNumber::Literal(num) => num.to_string(),
            SnailNumber::Pair(sn1, sn2) => format!("[{},{}]", sn1.serialize(), sn2.serialize()),
        }
    }*/
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_parse {
        use super::*;

        #[test]
        fn parse_simple() {
            let s = "[1,2]";
            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Literal(1)),
                Box::new(SnailNumber::Literal(2)),
            );
            assert_eq!(expected, SnailNumber::new(s));
        }

        #[test]
        fn parse_complicated() {
            let s = "[[[1,2],[3,4]],5]";
            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Pair(
                        Box::new(SnailNumber::Literal(1)),
                        Box::new(SnailNumber::Literal(2)),
                    )),
                    Box::new(SnailNumber::Pair(
                        Box::new(SnailNumber::Literal(3)),
                        Box::new(SnailNumber::Literal(4)),
                    )),
                )),
                Box::new(SnailNumber::Literal(5)),
            );
            assert_eq!(expected, SnailNumber::new(s));
        }
    }

    mod test_add {
        use super::*;

        #[test]
        fn add_two() {
            let sn1 = SnailNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
            let sn2 = SnailNumber::new("[1,1]");
            let expected = SnailNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
            assert_eq!(expected, SnailNumber::add(sn1, sn2));
        }
    }

    mod test_add_multiple {
        use super::*;

        #[test]
        fn add_multiple_1() {
            let numbers = vec![
                SnailNumber::new("[1,1]"),
                SnailNumber::new("[2,2]"),
                SnailNumber::new("[3,3]"),
                SnailNumber::new("[4,4]"),
            ];
            let expected = SnailNumber::new("[[[[1,1],[2,2]],[3,3]],[4,4]]");
            assert_eq!(expected, SnailNumber::add_multiple(numbers));
        }

        #[test]
        fn add_multiple_2() {
            let numbers = vec![
                SnailNumber::new("[1,1]"),
                SnailNumber::new("[2,2]"),
                SnailNumber::new("[3,3]"),
                SnailNumber::new("[4,4]"),
                SnailNumber::new("[5,5]"),
            ];
            let expected = SnailNumber::new("[[[[3,0],[5,3]],[4,4]],[5,5]]");
            assert_eq!(expected, SnailNumber::add_multiple(numbers));
        }

        #[test]
        fn add_multiple_3() {
            let numbers = vec![
                SnailNumber::new("[1,1]"),
                SnailNumber::new("[2,2]"),
                SnailNumber::new("[3,3]"),
                SnailNumber::new("[4,4]"),
                SnailNumber::new("[5,5]"),
                SnailNumber::new("[6,6]"),
            ];
            let expected = SnailNumber::new("[[[[5,0],[7,4]],[5,5]],[6,6]]");
            assert_eq!(expected, SnailNumber::add_multiple(numbers));
        }

        #[test]
        fn add_multiple_4() {
            let numbers = vec![
                SnailNumber::new("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
                SnailNumber::new("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
                SnailNumber::new("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
                SnailNumber::new("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
                SnailNumber::new("[7,[5,[[3,8],[1,4]]]]"),
                SnailNumber::new("[[2,[2,2]],[8,[8,1]]]"),
                SnailNumber::new("[2,9]"),
                SnailNumber::new("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
                SnailNumber::new("[[[5,[7,4]],7],1]"),
                SnailNumber::new("[[[[4,2],2],6],[8,7]]"),
            ];
            let expected =
                SnailNumber::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
            assert_eq!(expected, SnailNumber::add_multiple(numbers));
        }
    }

    mod test_explode {
        use super::*;

        #[test]
        fn explode_1() {
            let expected = SnailNumber::new("[[[[0,9],2],3],4]");
            let mut actual = SnailNumber::new("[[[[[9,8],1],2],3],4]");
            actual.explode();
            assert_eq!(expected, actual)
        }

        #[test]
        fn explode_2() {
            let expected = SnailNumber::new("[7,[6,[5,[7,0]]]]");
            let mut actual = SnailNumber::new("[7,[6,[5,[4,[3,2]]]]]");
            actual.explode();
            assert_eq!(expected, actual)
        }

        #[test]
        fn explode_3() {
            let expected = SnailNumber::new("[[6,[5,[7,0]]],3]");
            let mut actual = SnailNumber::new("[[6,[5,[4,[3,2]]]],1]");
            actual.explode();
            assert_eq!(expected, actual)
        }

        #[test]
        fn explode_4() {
            let expected = SnailNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
            let mut actual = SnailNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
            actual.explode();
            assert_eq!(expected, actual)
        }

        #[test]
        fn explode_5() {
            let expected = SnailNumber::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
            let mut actual = SnailNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
            actual.explode();
            assert_eq!(expected, actual)
        }
    }

    mod test_split {
        use super::*;

        #[test]
        fn nothing_to_do() {
            let expected = SnailNumber::Literal(5);
            let mut actual = SnailNumber::Literal(5);
            actual.split();
            assert_eq!(expected, actual);
        }

        #[test]
        fn simple_odd_literal() {
            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Literal(5)),
                Box::new(SnailNumber::Literal(6)),
            );
            let mut actual = SnailNumber::Literal(11);
            actual.split();
            assert_eq!(expected, actual);
        }

        #[test]
        fn simple_even_literal() {
            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Literal(5)),
                Box::new(SnailNumber::Literal(5)),
            );
            let mut actual = SnailNumber::Literal(10);
            actual.split();
            assert_eq!(expected, actual);
        }

        #[test]
        fn one_nested_literal() {
            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Literal(7)),
                    Box::new(SnailNumber::Literal(8)),
                )),
                Box::new(SnailNumber::Literal(6)),
            );
            let mut actual = SnailNumber::Pair(
                Box::new(SnailNumber::Literal(15)),
                Box::new(SnailNumber::Literal(6)),
            );
            actual.split();
            assert_eq!(expected, actual);
        }

        #[test]
        fn two_nested_literal() {
            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Literal(7)),
                    Box::new(SnailNumber::Literal(8)),
                )),
                Box::new(SnailNumber::Literal(15)),
            );
            let mut actual = SnailNumber::Pair(
                Box::new(SnailNumber::Literal(15)),
                Box::new(SnailNumber::Literal(15)),
            );
            actual.split();
            assert_eq!(expected, actual);

            let expected = SnailNumber::Pair(
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Literal(7)),
                    Box::new(SnailNumber::Literal(8)),
                )),
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Literal(7)),
                    Box::new(SnailNumber::Literal(8)),
                )),
            );
            actual.split();
            assert_eq!(expected, actual);
        }
    }

    mod test_magnitude {
        use super::*;

        #[test]
        fn magnitude_1() {
            let s = "[[1,2],[[3,4],5]]";
            let sn = SnailNumber::new(s);
            assert_eq!(143, sn.magnitude());
        }

        #[test]
        fn magnitude_2() {
            let s = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
            let sn = SnailNumber::new(s);
            assert_eq!(1384, sn.magnitude());
        }

        #[test]
        fn magnitude_3() {
            let s = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
            let sn = SnailNumber::new(s);
            assert_eq!(445, sn.magnitude());
        }

        #[test]
        fn magnitude_4() {
            let s = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
            let sn = SnailNumber::new(s);
            assert_eq!(791, sn.magnitude());
        }

        #[test]
        fn magnitude_5() {
            let s = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
            let sn = SnailNumber::new(s);
            assert_eq!(1137, sn.magnitude());
        }

        #[test]
        fn magnitude_6() {
            let s = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
            let sn = SnailNumber::new(s);
            assert_eq!(3488, sn.magnitude());
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(4140, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3993, part2());
    }
}
