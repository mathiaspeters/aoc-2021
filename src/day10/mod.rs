pub fn day10() {
    println!("Result 10-1: {}", part1());
    println!("Result 10-2: {}", part2());
}

pub fn part1() -> usize {
    process()
        .iter()
        .filter_map(|d| match d {
            Delimeters::Incomplete(_) => None,
            Delimeters::Corrupted(s) => Some(*s),
        })
        .sum()
}

pub fn part2() -> usize {
    let mut scores = process()
        .iter()
        .filter_map(|d| match d {
            Delimeters::Incomplete(open) => Some(calculate_autocomplete_score(open)),
            Delimeters::Corrupted(_) => None,
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn calculate_autocomplete_score(open: &[char]) -> usize {
    let mut result = 0;
    open.iter().rev().for_each(|c| {
        result *= 5;
        result += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        }
    });
    result
}

fn process() -> Vec<Delimeters> {
    let input = input();
    let mut output = Vec::with_capacity(input.len());
    input.iter().for_each(|line| {
        let mut open = vec![];
        let mut corrupted_score = None;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    open.push(c);
                }
                ')' => {
                    if *open.last().unwrap() == '(' {
                        open.pop();
                    } else {
                        corrupted_score = Some(3);
                        break;
                    }
                }
                ']' => {
                    if *open.last().unwrap() == '[' {
                        open.pop();
                    } else {
                        corrupted_score = Some(57);
                        break;
                    }
                }
                '}' => {
                    if *open.last().unwrap() == '{' {
                        open.pop();
                    } else {
                        corrupted_score = Some(1197);
                        break;
                    }
                }
                '>' => {
                    if *open.last().unwrap() == '<' {
                        open.pop();
                    } else {
                        corrupted_score = Some(25137);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(score) = corrupted_score {
            output.push(Delimeters::Corrupted(score));
        } else {
            output.push(Delimeters::Incomplete(open));
        }
    });
    output
}

enum Delimeters {
    Incomplete(Vec<char>),
    Corrupted(usize),
}

fn input() -> Vec<&'static str> {
    raw_input().split('\n').collect()
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

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, part2());
    }
}
