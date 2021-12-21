pub fn day19() {
    println!("Result 19-1: {}", part1());
    println!("Result 19-2: {}", part2());
}

pub fn part1() -> usize {
    process(Part::One)
}

pub fn part2() -> usize {
    process(Part::Two)
}

enum Part {
    One,
    Two,
}

fn process(part: Part) -> usize {
    let mut scanners = input();
    let mut scanner_positions = vec![(0_i16, 0_i16, 0_i16); scanners.len()];
    let common = get_common(&scanners);
    common.iter().for_each(|((i, j), c)| {
        let i = *i;
        let j = *j;
        let mut s2 = scanners[j].clone();
        let offsets = get_scanner_position(&scanners[i], &s2, &c);
        offset_scanner(&mut s2, offsets);
        std::mem::swap(&mut scanners[j], &mut s2);
        scanner_positions[j] = (offsets[0].2, offsets[1].2, offsets[2].2);
    });
    match part {
        Part::One => count_beacons(&scanners),
        Part::Two => get_biggest_scanner_distance(&scanner_positions),
    }
}

fn count_beacons(scanners: &[Scanner]) -> usize {
    let mut beacons: Vec<(i16, i16, i16)> = vec![];
    scanners.iter().for_each(|s| {
        for i in 0..s.readings[0].len() {
            let b = (s.readings[0][i], s.readings[1][i], s.readings[2][i]);
            beacons.push(b);
        }
    });
    beacons.sort_unstable();
    beacons.dedup();
    beacons.len()
}

fn get_biggest_scanner_distance(scanner_positions: &[(i16, i16, i16)]) -> usize {
    let mut biggest_distance = 0;
    for i in 0..scanner_positions.len() {
        let a = scanner_positions[i];
        for j in 1..scanner_positions.len() {
            let b = scanner_positions[j];
            let distance = ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()) as usize;
            if distance > biggest_distance {
                biggest_distance = distance;
            }
        }
    }
    biggest_distance
}

fn get_scanner_position(
    s1: &Scanner,
    s2: &Scanner,
    common: &[(usize, usize)],
) -> [(usize, i16, i16); 3] {
    let x = (0..3)
        .find(|i| check_axis(&s1.readings[0], &s2.readings[*i], common))
        .map(|i| {
            let (a, b) = get_diff(&s1.readings[0], &s2.readings[i], common);
            (i, a, b)
        })
        .unwrap();
    let y = (0..3)
        .find(|i| check_axis(&s1.readings[1], &s2.readings[*i], common))
        .map(|i| {
            let (a, b) = get_diff(&s1.readings[1], &s2.readings[i], common);
            (i, a, b)
        })
        .unwrap();
    let z = (0..3)
        .find(|i| check_axis(&s1.readings[2], &s2.readings[*i], common))
        .map(|i| {
            let (a, b) = get_diff(&s1.readings[2], &s2.readings[i], common);
            (i, a, b)
        })
        .unwrap();

    [x, y, z]
}

fn offset_scanner(s: &mut Scanner, offsets: [(usize, i16, i16); 3]) {
    let (mx, x_mul, x) = offsets[0];
    let (my, y_mul, y) = offsets[1];
    let (mz, z_mul, z) = offsets[2];

    for i in 0..s.readings[0].len() {
        let x_tmp = s.readings[mx][i] * x_mul + x;
        let y_tmp = s.readings[my][i] * y_mul + y;
        let z_tmp = s.readings[mz][i] * z_mul + z;
        s.readings[0][i] = x_tmp;
        s.readings[1][i] = y_tmp;
        s.readings[2][i] = z_tmp;
    }
}

fn get_diff(r: &[i16], a: &[i16], m: &[(usize, usize)]) -> (i16, i16) {
    let ref1 = a[m[0].1] - r[m[0].0];
    let ref2 = a[m[0].1] + r[m[0].0];
    for i in 1..m.len() {
        let cmp1 = a[m[i].1] - r[m[i].0];
        let cmp2 = a[m[i].1] + r[m[i].0];
        if cmp1 == ref1 && cmp2 == ref2 {
            continue;
        } else if cmp1 == ref1 {
            return (1, ref1 * -1);
        } else {
            return (-1, cmp2);
        }
    }
    panic!("All of them are the same...");
}

fn check_axis(r: &[i16], a: &[i16], m: &[(usize, usize)]) -> bool {
    (r[m[0].0] - r[m[1].0]).abs() == (a[m[0].1] - a[m[1].1]).abs()
}

fn get_common(scanners: &[Scanner]) -> Vec<((usize, usize), Vec<(usize, usize)>)> {
    let mut output = vec![];
    let mut remaining = (1..scanners.len()).collect::<Vec<_>>();
    let mut open = vec![0];
    let mut closed = vec![];
    while !remaining.is_empty() {
        let current = open.remove(0);
        let mut to_remove = vec![];
        remaining.iter().enumerate().for_each(|(index, r)| {
            if let Some(c) = check_common(&scanners[current], &scanners[*r]) {
                to_remove.push(index);
                open.push(*r);
                output.push(((current, *r), c));
            }
        });
        to_remove.into_iter().rev().for_each(|i| {
            remaining.remove(i);
        });
        closed.push(current);
    }
    output
}

fn check_common(s1: &Scanner, s2: &Scanner) -> Option<Vec<(usize, usize)>> {
    for i in 0..s1.permutations.len() {
        for j in 0..s2.permutations.len() {
            let res = check_permutations(&s1.permutations[i], &s2.permutations[j]);
            if res.len() >= 12 {
                return Some(res);
            }
        }
    }
    None
}

fn check_permutations(p1: &[[i16; 3]], p2: &[[i16; 3]]) -> Vec<(usize, usize)> {
    let mut common = vec![];
    p1.iter().enumerate().for_each(|(i, v1)| {
        for j in 0..p2.len() {
            if v1.iter().all(|v3| p2[j].contains(v3)) {
                common.push((i, j));
                break;
            }
        }
    });
    common
}

fn input() -> Vec<Scanner> {
    let mut output = Vec::with_capacity(35);
    let mut scanner = Scanner::default();
    raw_input().lines().skip(1).for_each(|line| {
        if line.starts_with("---") {
            let mut s = Scanner::default();
            std::mem::swap(&mut scanner, &mut s);
            s.init_permutations();
            output.push(s);
        } else if !line.is_empty() {
            let r = line
                .split(',')
                .map(|r| r.parse::<i16>().unwrap())
                .collect::<Vec<_>>();
            scanner.add_reading(&r);
        }
    });
    scanner.init_permutations();
    output.push(scanner);
    output
}

#[derive(Debug, Default, Clone)]
struct Scanner {
    readings: [Vec<i16>; 3],
    permutations: Vec<Vec<[i16; 3]>>,
}

impl Scanner {
    pub fn add_reading(&mut self, r: &[i16]) {
        self.readings[0].push(r[0]);
        self.readings[1].push(r[1]);
        self.readings[2].push(r[2]);
    }

    pub fn init_permutations(&mut self) {
        for i in 0..self.readings[0].len() {
            let mut p = Vec::with_capacity(self.readings[0].len());
            for j in 0..self.readings[0].len() {
                let x = (self.readings[0][j] - self.readings[0][i]).abs();
                let y = (self.readings[1][j] - self.readings[1][i]).abs();
                let z = (self.readings[2][j] - self.readings[2][i]).abs();
                p.push([x, y, z]);
            }
            self.permutations.push(p);
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
        assert_eq!(79, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3621, part2());
    }
}
