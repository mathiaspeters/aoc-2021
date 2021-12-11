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
    let mut population = [0_u32; 9];
    raw_input().chars().filter(|c| *c != ',').for_each(|c| {
        population[super::util::char_to_u8(c).unwrap() as usize] += 1;
    });
    (0..days).for_each(|_| {
        let reproducing = population[0];
        population[7] += population[0];
        (1..9).for_each(|i| population[i - 1] = population[i]);
        population[8] = reproducing;
    });
    population
        .into_iter()
        .fold(0_usize, |acc, val| acc + val as usize)
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
