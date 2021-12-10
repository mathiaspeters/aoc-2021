pub fn day1() {
    println!("Result  1-1: {}", part1());
    println!("Result  1-2: {}", part2());
}

pub fn part1() -> usize {
    input()
        .windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum()
}

pub fn part2() -> usize {
    input()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<u16>>()
        .windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum()
}

fn input() -> Vec<u16> {
    raw_input()
        .split_ascii_whitespace()
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.parse::<u16>().unwrap())
            }
        })
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
        assert_eq!(7, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, part2());
    }
}
