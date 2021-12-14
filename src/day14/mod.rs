use std::collections::HashMap;

pub fn day14() {
    println!("Result 14-1: {}", part1());
    println!("Result 14-2: {}", part2());
}

pub fn part1() -> usize {
    process(10)
}

pub fn part2() -> usize {
    process(40)
}

fn process(iterations: usize) -> usize {
    let input = input();
    let (mut histogram, sub_rules, char_map, last_char) = input;
    for _ in 0..iterations {
        let mut new_histogram = vec![0; histogram.len()];
        for i in 0..histogram.len() {
            if let Some((left, right)) = sub_rules[i] {
                new_histogram[left] += histogram[i];
                new_histogram[right] += histogram[i];
            }
        }
        histogram = new_histogram;
    }
    let mut map = HashMap::new();
    for i in 0..char_map.len() {
        *map.entry(char_map[i].0).or_insert(0) += histogram[i];
    }
    *map.entry(last_char).or_insert(0) += 1;
    let mut counts = map.into_values().collect::<Vec<_>>();
    counts.sort_unstable();
    counts[counts.len() - 1] - counts[0]
}

fn input() -> (
    Vec<usize>,
    Vec<Option<(usize, usize)>>,
    Vec<(char, char)>,
    char,
) {
    let mut char_mapper = CharMapper::default();
    let mut lines = raw_input().lines();
    let polymer = lines.next().unwrap().chars().collect::<Vec<_>>();
    let last_char = polymer[polymer.len() - 1];
    let polymer = polymer
        .windows(2)
        .map(|c| char_mapper.map(c[0], c[1]))
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let mut sub_rules = vec![];
    lines.for_each(|line| {
        let mut chars = line.chars().filter(|c| c.is_ascii_alphabetic());
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        let c3 = chars.next().unwrap();
        let index = char_mapper.map(c1, c2);
        let first = char_mapper.map(c1, c3);
        let second = char_mapper.map(c3, c2);
        if sub_rules.len() < index + 1 {
            sub_rules.resize(index + 1, None);
        }
        sub_rules[index] = Some((first, second));
    });
    let mut histogram = vec![0_usize; char_mapper.chars.len()];
    polymer.into_iter().for_each(|value| histogram[value] += 1);
    (histogram, sub_rules, char_mapper.chars, last_char)
}

#[derive(Default)]
struct CharMapper {
    chars: Vec<(char, char)>,
}

impl CharMapper {
    pub fn map(&mut self, c1: char, c2: char) -> usize {
        if let Some(index) =
            self.chars
                .iter()
                .enumerate()
                .find_map(|(index, v)| if *v == (c1, c2) { Some(index) } else { None })
        {
            index
        } else {
            let output = self.chars.len();
            self.chars.push((c1, c2));
            output
        }
    }
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
        assert_eq!(1588, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2188189693529, part2());
    }
}
