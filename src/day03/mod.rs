pub fn day3() {
    println!("Result  3-1: {}", part1());
    println!("Result  3-2: {}", part2());
}

pub fn part1() -> usize {
    let relative_counts = relative_counts(&input());
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut bit_value = 1;
    relative_counts.iter().rev().for_each(|a| {
        if *a > 0 {
            gamma += bit_value;
        } else {
            epsilon += bit_value;
        }
        bit_value *= 2;
    });
    gamma * epsilon
}

pub fn part2() -> usize {
    let input = input();
    let values = (0..2)
        .map(|iteration| {
            let mut position = 0;
            let mut intermediate = input.clone();
            let mut counts = relative_counts(&intermediate);
            while intermediate.len() > 1 {
                intermediate = intermediate
                    .into_iter()
                    .filter(|val| {
                        let c = val.chars().nth(position).unwrap();
                        match (c, iteration) {
                            // Keep most common digit
                            ('1', 0) => counts[position] >= 0,
                            ('0', 0) => counts[position] < 0,
                            // Keep least common digit
                            ('1', 1) => counts[position] < 0,
                            ('0', 1) => counts[position] >= 0,
                            _ => panic!(),
                        }
                    })
                    .collect::<Vec<_>>();
                counts = relative_counts(&intermediate);
                position += 1;
            }
            let final_str = intermediate[0];
            let mut bit_value = 1;
            let mut result = 0;
            final_str.chars().rev().for_each(|c| {
                if c == '1' {
                    result += bit_value
                }
                bit_value *= 2;
            });
            result
        })
        .collect::<Vec<_>>();
    values[0] * values[1]
}

fn relative_counts(input: &[&str]) -> Vec<i32> {
    let mut acc = vec![0; input[0].len()];
    input.iter().for_each(|b| {
        b.chars().enumerate().for_each(|(pos, val)| {
            if val == '0' {
                acc[pos] -= 1;
            } else {
                acc[pos] += 1;
            }
        });
    });
    acc
}

fn input() -> Vec<&'static str> {
    raw_input()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
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
        assert_eq!(198, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(230, part2());
    }
}
