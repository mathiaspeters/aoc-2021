pub fn day8() {
    println!("Result  8-1: {}", part1());
    println!("Result  8-2: {}", part2());
}

pub fn part1() -> usize {
    let to_match = [2_usize, 3, 4, 7];
    input()
        .iter()
        .flat_map(|(_, output)| {
            output
                .split_ascii_whitespace()
                .filter_map(|o| {
                    if to_match.contains(&o.len()) {
                        Some(1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .count()
}

pub fn part2() -> usize {
    input()
        .iter()
        .map(|(input, output)| {
            let input = input
                .split_ascii_whitespace()
                .filter(|i| !i.is_empty())
                .collect::<Vec<_>>();
            let output = output
                .split_ascii_whitespace()
                .filter(|o| !o.is_empty())
                .collect::<Vec<_>>();
            let dict = build_dictionary(&input);
            decode(&output, dict)
        })
        .sum()
}

fn build_dictionary(input: &[&'static str]) -> [&'static str; 10] {
    let mut output = [""; 10];
    let mut five = vec![];
    let mut six = vec![];
    input.into_iter().for_each(|s| {
        if s.len() == 2 {
            output[1] = s;
        } else if s.len() == 3 {
            output[7] = s;
        } else if s.len() == 4 {
            output[4] = s;
        } else if s.len() == 5 {
            five.push(*s);
        } else if s.len() == 6 {
            six.push(*s);
        } else {
            output[8] = s;
        }
    });
    let (to_remove, value) = five
        .iter()
        .enumerate()
        .find(|(_, s)| output[1].chars().all(|c| s.contains(c)))
        .unwrap();
    output[3] = value;
    five.remove(to_remove);
    six.iter().for_each(|s| {
        if !output[1].chars().all(|c| s.contains(c)) {
            output[6] = s;
        } else if output[3].chars().all(|c| s.contains(c)) {
            output[9] = s;
        } else {
            output[0] = s;
        }
    });
    five.iter().for_each(|s| {
        if s.chars().all(|c| output[6].contains(c)) {
            output[5] = s;
        } else {
            output[2] = s;
        }
    });

    output
}

fn decode(output: &[&'static str], dictionary: [&'static str; 10]) -> usize {
    let mut result = 0;
    let mut multiplier = 1;
    output.iter().rev().for_each(|o| {
        let value = if o.len() == 2 {
            1
        } else if o.len() == 3 {
            7
        } else if o.len() == 4 {
            4
        } else if o.len() == 5 {
            if o.chars().all(|c| dictionary[2].contains(c)) {
                2
            } else if o.chars().all(|c| dictionary[3].contains(c)) {
                3
            } else {
                5
            }
        } else if o.len() == 6 {
            if o.chars().all(|c| dictionary[0].contains(c)) {
                0
            } else if o.chars().all(|c| dictionary[6].contains(c)) {
                6
            } else {
                9
            }
        } else {
            8
        };
        result += value * multiplier;
        multiplier *= 10;
    });
    result
}

fn input() -> Vec<(&'static str, &'static str)> {
    raw_input()
        .split('\n')
        .map(|line| {
            let mut s = line.split('|');
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect()
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
        assert_eq!(26, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(61229, part2());
    }
}
