use std::collections::HashMap;

pub fn day5() {
    println!("Result  5-1: {}", part1());
    println!("Result  5-2: {}", part2());
}

pub fn part1() -> usize {
    process(false) as usize
}

pub fn part2() -> usize {
    process(true) as usize
}
fn process(diagonals: bool) -> u32 {
    let mut grid: HashMap<(u16, u16), u8> = HashMap::new();
    input().chunks(2).for_each(|chunk| {
        let start = coordinate(chunk[0]);
        let end = coordinate(chunk[1]);
        let mut add_entries = |index: usize, x: Option<u16>, y: Option<u16>| {
            let range = if start[index] < end[index] {
                start[index]..=end[index]
            } else {
                end[index]..=start[index]
            };
            for i in range {
                let key = if x.is_some() {
                    (x.unwrap(), i)
                } else {
                    (i, y.unwrap())
                };
                *grid.entry(key).or_insert(0) += 1;
            }
        };
        if start[0] == end[0] {
            add_entries(1, Some(start[0]), None);
        } else if start[1] == end[1] {
            add_entries(0, None, Some(start[1]));
        } else if diagonals {
            let mut dirx = 0;
            let mut diry = 0;
            let xc = if start[0] < end[0] {
                (start[0]..=end[0]).collect::<Vec<_>>()
            } else {
                dirx = 1;
                (end[0]..=start[0]).collect::<Vec<_>>()
            };
            let yc = if start[1] < end[1] {
                (start[1]..=end[1]).collect::<Vec<_>>()
            } else {
                diry = 1;
                (end[1]..=start[1]).collect::<Vec<_>>()
            };
            if dirx == diry {
                xc.iter().zip(yc.iter()).for_each(|(x, y)| {
                    *grid.entry((*x, *y)).or_insert(0) += 1;
                });
            } else {
                xc.iter().zip(yc.iter().rev()).for_each(|(x, y)| {
                    *grid.entry((*x, *y)).or_insert(0) += 1;
                });
            }
        }
    });
    grid.iter().map(|(_, v)| if *v > 1 { 1 } else { 0 }).sum()
}

fn coordinate(s: &str) -> [u16; 2] {
    let parts = s
        .split(",")
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    [parts[0], parts[1]]
}

fn input() -> Vec<&'static str> {
    raw_input()
        .split(" -> ")
        .flat_map(|s| s.split_ascii_whitespace())
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
        assert_eq!(5, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2());
    }
}
