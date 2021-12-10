pub fn day2() {
    println!("Result  2-1: {}", part1());
    println!("Result  2-2: {}", part2());
}

pub fn part1() -> usize {
    let mut depth = 0;
    let mut horizontal_distance = 0;
    input().chunks(2).for_each(|window| {
        let direction = window[0];
        let distance = window[1].parse::<i32>().unwrap();
        if direction.starts_with('f') {
            horizontal_distance += distance;
        } else if direction.starts_with('u') {
            depth -= distance;
        } else {
            depth += distance;
        }
    });
    (depth * horizontal_distance) as usize
}

pub fn part2() -> usize {
    let mut depth = 0;
    let mut horizontal_distance = 0;
    let mut aim = 0;
    input().chunks(2).for_each(|window| {
        let direction = window[0];
        let distance = window[1].parse::<i32>().unwrap();
        if direction.starts_with('f') {
            horizontal_distance += distance;
            depth += aim * distance;
        } else if direction.starts_with('u') {
            aim -= distance;
        } else {
            aim += distance;
        }
    });
    (depth * horizontal_distance) as usize
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
        assert_eq!(150, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(900, part2());
    }
}
