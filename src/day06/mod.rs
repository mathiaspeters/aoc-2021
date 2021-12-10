pub fn day6() {
    println!("Result  6-1: {}", part1());
    println!("Result  6-2: {}", part2());
}

pub fn part1() -> usize {
    process(80)
}

pub fn part2() -> usize {
    process(256)
}
fn process(days: usize) -> usize {
    let mut population = [0_usize; 9];
    input().iter().for_each(|f| population[*f] += 1);
    for _ in 0..days {
        let mut new_population = [0_usize; 9];
        for i in 1..9 {
            new_population[i - 1] = population[i]
        }
        new_population[6] += population[0];
        new_population[8] += population[0];
        population = new_population;
    }
    let mut result = 0;
    for p in population {
        result += p;
    }
    result as usize
}

fn input() -> Vec<usize> {
    raw_input()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
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
        assert_eq!(5934, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539, part2());
    }
}
