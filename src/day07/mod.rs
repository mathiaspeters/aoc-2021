pub fn day7() {
    println!("Result  7-1: {}", part1());
    println!("Result  7-2: {}", part2());
}

pub fn part1() -> usize {
    let mut best = (0, 0);
    let crabs: Vec<u32> = crab_distribution();
    for i in 0..crabs.len() {
        let mut cumulative_distance = 0;
        crabs.iter().enumerate().for_each(|(crab_pos, crab)| {
            cumulative_distance += (crab_pos as i32 - i as i32).abs() * (*crab as i32);
        });
        if i == 0 || cumulative_distance < best.1 {
            best = (i, cumulative_distance);
        }
    }
    best.1 as usize
}

pub fn part2() -> usize {
    let mut best = (0, 0);
    let crabs: Vec<u32> = crab_distribution();
    for i in 0..crabs.len() {
        let mut cumulative_distance = 0;
        crabs.iter().enumerate().for_each(|(crab_pos, crab)| {
            let crab_count = *crab as i32;
            let distance_per_crab = (crab_pos as i32 - i as i32).abs();
            let cost_per_crab = if distance_per_crab & 1 == 1 {
                distance_per_crab * (((distance_per_crab - 1) / 2) + 1)
            } else {
                (distance_per_crab + 1) * (distance_per_crab / 2)
            };
            cumulative_distance += cost_per_crab * crab_count;
        });
        if i == 0 || cumulative_distance < best.1 {
            best = (i, cumulative_distance);
        }
    }
    best.1 as usize
}

fn crab_distribution() -> Vec<u32> {
    let raw_crabs = raw_input()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut distribution: Vec<u32> = vec![];
    raw_crabs.iter().for_each(|crab| {
        if *crab >= distribution.len() {
            distribution.resize_with(*crab + 1, Default::default);
        }
        distribution[*crab] += 1;
    });
    distribution
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
        assert_eq!(37, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, part2());
    }
}
