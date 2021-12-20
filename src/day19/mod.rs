pub fn day19() {
    println!("Result 19-1: {}", part1());
    println!("Result 19-2: {}", part2());
}

pub fn part1() -> usize {
    let mut scanners = input();
    let mut common: Vec<Vec<(usize, Vec<(usize, usize)>)>> = vec![];
    for i in 0..scanners.len() {
        let mut com = vec![];
        for j in i + 1..scanners.len() {
            if let Some(c) = check_common(&scanners[i], &scanners[j]) {
                com.push((j, c));
            }
        }
        common.push(com);
    }
    dbg!(&common);
    common.iter().enumerate().for_each(|(i, c)| {
        c.iter().for_each(|(j, c2)| {
            let j = if i == 2 { 2 } else { *j };
            let i = if i == 2 { 4 } else { i };
            let mapped_c2 = c2.iter().map(|(a, b)| (*b, *a)).collect::<Vec<_>>();
            let c2 = if j == 2 { &mapped_c2 } else { c2 };

            //println!("{}, {}", i, j);
            let mut s2 = scanners[j].clone();
            /*println!("\nCommon {}-{}", i, j);
            c2.iter().for_each(|(a, b)| println!("{},{}", a, b));
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
            }*/
            calculate_offset(&scanners[i], &mut s2, &c2);
            std::mem::swap(&mut scanners[j], &mut s2);
        });
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
    let (m1, m2, m3) = if check_axis(&s1.readings[0], &s2.readings[0], common) {
        println!("x");
        (0, 1, 2)
    } else if check_axis(&s1.readings[0], &s2.readings[1], common) {
        println!("y");
        (1, 2, 0)
    } else {
        println!("z");
        (2, 0, 1)
    };
    let x_rev = common.iter().fold(true, |acc, (a, b)| {
        acc && s1.readings[0][*a].signum() != s2.readings[m1][*b].signum()
    });
    let y_rev = common.iter().fold(true, |acc, (a, b)| {
        acc && s1.readings[1][*a].signum() != s2.readings[m2][*b].signum()
    });
    let z_rev = common.iter().fold(true, |acc, (a, b)| {
        acc && s1.readings[2][*a].signum() != s2.readings[m3][*b].signum()
    });
    dbg!(
        x_rev,
        s1.readings[0][common[0].0],
        s2.readings[m1][common[0].1],
        y_rev,
        s1.readings[1][common[0].0],
        s2.readings[m2][common[0].1],
        z_rev,
        s1.readings[2][common[0].0],
        s2.readings[m3][common[0].1],
    );
    let mut x = s1.readings[0][common[0].0];
    let mut y = s1.readings[1][common[0].0];
    let mut z = s1.readings[2][common[0].0];
    if x_rev {
        x += s2.readings[m1][common[0].1];
    } else {
        x -= s2.readings[m1][common[0].1];
    }
    if y_rev {
        y -= s2.readings[m2][common[0].1];
    } else {
        y += s2.readings[m2][common[0].1];
    }
    if z_rev && y_rev == x_rev {
        z += s2.readings[m3][common[0].1];
    } else {
        z -= s2.readings[m3][common[0].1];
    }
    println!("({},{},{})", x, y, z);

    let xm = if x_rev { -1 } else { 1 };
    let zm = if z_rev { -1 } else { 1 };
    for i in 0..s2.readings[0].len() {
        let mut new_x = s2.readings[m1][i] + x * xm;
        if x_rev {
            new_x *= xm;
        }
        let new_y = s2.readings[m2][i] + y;
        let mut new_z = s2.readings[m3][i] + z * zm;
        if z_rev {
            new_z *= zm;
        }
        s2.readings[0][i] = new_x;
        s2.readings[1][i] = new_y;
        s2.readings[2][i] = new_z;
    }
}

fn check_axis(r: &[i16], a: &[i16], m: &[(usize, usize)]) -> bool {
    (r[m[0].0] - r[m[1].0]).abs() == (a[m[0].1] - a[m[1].1]).abs()
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
        assert_eq!(0, part2());
    }
}
