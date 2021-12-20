pub fn day19() {
    println!("Result 19-1: {}", part1());
    println!("Result 19-2: {}", part2());
}

pub fn part1() -> usize {
    let mut scanners = input();
    let common = get_common(&scanners);
    common.iter().for_each(|((i, j), c)| {
        println!("\nCommon {}-{}", i, j);
        c.iter().for_each(|(a, b)| println!("{},{}", a, b));
    });
    common.iter().for_each(|((i, j), c)| {
        let i = *i;
        let j = *j;
        println!("\nFinding common beacons for: {}, {}", i, j);
        let mut s2 = scanners[j].clone();
        println!("\nScanner {}", i);
        for index in 0..scanners[i].readings[0].len() {
            println!(
                "{},{},{}",
                scanners[i].readings[0][index],
                scanners[i].readings[1][index],
                scanners[i].readings[2][index]
            );
        }
        println!("\nScanner {}", j);
        for index in 0..scanners[0].readings[0].len() {
            println!(
                "{},{},{}",
                s2.readings[0][index], s2.readings[1][index], s2.readings[2][index]
            );
        }
        calculate_offset(&scanners[i], &mut s2, &c);
        std::mem::swap(&mut scanners[j], &mut s2);
    });
    let mut beacons: Vec<(i16, i16, i16)> = vec![];
    scanners.iter().for_each(|s| {
        for i in 0..s.readings[0].len() {
            let b = (s.readings[0][i], s.readings[1][i], s.readings[2][i]);
            if !beacons.contains(&b) {
                beacons.push(b);
            }
        }
    });
    beacons.sort_unstable();
    beacons.len()
}

pub fn part2() -> usize {
    raw_input().len()
}

fn calculate_offset(s1: &Scanner, s2: &mut Scanner, common: &[(usize, usize)]) {
    let (x_mul, x) = (0..3)
        .find(|i| check_axis(&s1.readings[0], &s2.readings[*i], common))
        .map(|i| get_diff(&s1.readings[0], &s2.readings[i], common))
        .unwrap();
    let (y_mul, y) = (0..3)
        .find(|i| check_axis(&s1.readings[1], &s2.readings[*i], common))
        .map(|i| get_diff(&s1.readings[1], &s2.readings[i], common))
        .unwrap();
    let (z_mul, z) = (0..3)
        .find(|i| check_axis(&s1.readings[2], &s2.readings[*i], common))
        .map(|i| get_diff(&s1.readings[2], &s2.readings[i], common))
        .unwrap();
    println!("({},{}),({},{}),({},{})", x, x_mul, y, y_mul, z, z_mul);

    for i in 0..s2.readings[0].len() {
        s2.readings[0][i] -= x;
        s2.readings[0][i] *= x_mul;
        s2.readings[1][i] -= y;
        s2.readings[1][i] *= y_mul;
        s2.readings[2][i] -= z;
        s2.readings[2][i] *= z_mul;
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
            return (1, ref1);
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
        assert_eq!(0, part2());
    }
}
