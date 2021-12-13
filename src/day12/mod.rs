pub fn day12() {
    println!("Result 12-1: {}", part1());
    println!("Result 12-2: {}", part2());
}

pub fn part1() -> usize {
    count_paths(false)
}

pub fn part2() -> usize {
    count_paths(true)
}

fn count_paths(dup_allowed: bool) -> usize {
    let mapping = get_mapping();
    let mut num_paths = 0;
    let mut so_far = Vec::with_capacity(10);
    so_far.push(Cave::start());
    for node in &mapping[0] {
        build_path(*node, dup_allowed, &so_far, &mapping, &mut num_paths);
    }
    num_paths
}

fn build_path(
    next_value: Cave,
    dup_allowed: bool,
    so_far: &[Cave],
    mapping: &[Vec<Cave>],
    num_paths: &mut usize,
) {
    if next_value.value == 1 {
        *num_paths += 1;
    } else if next_value.value == 0 {
    } else if next_value.is_large || !so_far.contains(&next_value) {
        let mut so_far = so_far.to_vec();
        so_far.push(next_value);
        for node in &mapping[next_value.value as usize] {
            build_path(*node, dup_allowed, &so_far, &mapping, num_paths);
        }
    } else if dup_allowed {
        let mut so_far = so_far.to_vec();
        so_far.push(next_value);
        for node in &mapping[next_value.value as usize] {
            build_path(*node, false, &so_far, &mapping, num_paths);
        }
    }
}

fn get_mapping() -> Vec<Vec<Cave>> {
    let mut output_mapping = Vec::with_capacity(15);
    let mut mapping = vec![("start", Cave::start()), ("end", Cave::end())];
    raw_input().lines().for_each(|line| {
        let mut parts = line.split('-');
        let (start, end) = match (parts.next().unwrap(), parts.next().unwrap()) {
            ("start", "end") | ("end", "start") => (Cave::start(), Cave::end()),
            ("start", p) => (Cave::start(), make_cave(p, &mut mapping)),
            ("end", p) => (Cave::end(), make_cave(p, &mut mapping)),
            (p, "start") => (make_cave(p, &mut mapping), Cave::start()),
            (p, "end") => (make_cave(p, &mut mapping), Cave::end()),
            (p1, p2) => (make_cave(p1, &mut mapping), make_cave(p2, &mut mapping)),
        };
        let needed_size = std::cmp::max(start.value, end.value) as usize + 1;
        output_mapping.resize(
            std::cmp::max(output_mapping.len(), needed_size),
            Vec::with_capacity(15),
        );
        if end.value != 0 {
            output_mapping[start.value as usize].push(end);
        }
        if start.value != 0 {
            output_mapping[end.value as usize].push(start);
        }
    });
    output_mapping
}

fn make_cave<'a>(str_value: &'a str, mapping: &mut Vec<(&'a str, Cave)>) -> Cave {
    mapping
        .iter()
        .find_map(|(s, c)| if *s == str_value { Some(*c) } else { None })
        .unwrap_or_else(|| {
            let value = mapping.len() as u8;
            let cave = Cave::new(value, str_value.chars().all(|c| c.is_uppercase()));
            mapping.push((str_value, cave));
            cave
        })
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Cave {
    value: u8,
    is_large: bool,
}

impl Cave {
    pub fn new(value: u8, is_large: bool) -> Self {
        Self { value, is_large }
    }

    pub fn start() -> Self {
        Self {
            value: 0,
            is_large: false,
        }
    }

    pub fn end() -> Self {
        Self {
            value: 1,
            is_large: true,
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
        assert_eq!(10, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, part2());
    }
}
