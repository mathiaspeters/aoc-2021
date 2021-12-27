pub fn day22() {
    println!("Result 22-1: {}", part1());
    println!("Result 22-2: {}", part2());
}

pub fn part1() -> usize {
    let input = input();
    let input = limit(input, -50, 50);
    process(input)
}

pub fn part2() -> usize {
    let input = input();
    process(input)
}

fn process(input: Vec<(bool, Cuboid)>) -> usize {
    let mut is_on = Vec::with_capacity(input.len());
    let mut cuboids = Vec::with_capacity(input.len());
    input.iter().for_each(|(on, cuboid)| {
        is_on.push(*on);
        cuboids.push(*cuboid);
    });
    cuboids
        .iter()
        .enumerate()
        .map(|(i, cuboid)| {
            if is_on[i] {
                count_active(*cuboid, &cuboids[i + 1..])
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn count_active(cuboid: Cuboid, following: &[Cuboid]) -> usize {
    let mut intersecting = vec![];
    for i in 0..following.len() {
        match cuboid.sub_cube(following[i]) {
            Intersection::Full => return 0,
            Intersection::None => {}
            Intersection::Partial(c) => {
                if !intersecting.iter().any(|c2| c.contained_by(*c2)) {
                    intersecting.push(c)
                }
            }
        }
    }
    let exclude = intersecting
        .iter()
        .enumerate()
        .map(|(i, c)| count_active(*c, &intersecting[i + 1..]))
        .sum::<usize>();

    cuboid.count_active().saturating_sub(exclude)
}

#[derive(Copy, Clone, Debug)]
struct Cuboid {
    xr: (i32, i32),
    yr: (i32, i32),
    zr: (i32, i32),
}

enum Intersection {
    Full,
    None,
    Partial(Cuboid),
}

impl Cuboid {
    pub fn sub_cube(&self, other: Cuboid) -> Intersection {
        if self.contained_by(other) {
            Intersection::Full
        } else if self.intersecting(other) {
            Intersection::Partial(Cuboid {
                xr: (
                    std::cmp::max(self.xr.0, other.xr.0),
                    std::cmp::min(self.xr.1, other.xr.1),
                ),
                yr: (
                    std::cmp::max(self.yr.0, other.yr.0),
                    std::cmp::min(self.yr.1, other.yr.1),
                ),
                zr: (
                    std::cmp::max(self.zr.0, other.zr.0),
                    std::cmp::min(self.zr.1, other.zr.1),
                ),
            })
        } else {
            Intersection::None
        }
    }

    fn intersecting(&self, other: Cuboid) -> bool {
        let x = other.xr.0 <= self.xr.1 && other.xr.1 >= self.xr.0;
        let y = other.yr.0 <= self.yr.1 && other.yr.1 >= self.yr.0;
        let z = other.zr.0 <= self.zr.1 && other.zr.1 >= self.zr.0;
        x && y && z
    }

    fn contained_by(&self, other: Cuboid) -> bool {
        let x = other.xr.0 <= self.xr.0 && other.xr.1 >= self.xr.1;
        let y = other.yr.0 <= self.yr.0 && other.yr.1 >= self.yr.1;
        let z = other.zr.0 <= self.zr.0 && other.zr.1 >= self.zr.1;
        x && y && z
    }

    pub fn count_active(&self) -> usize {
        (self.xr.1 - self.xr.0 + 1) as usize
            * (self.yr.1 - self.yr.0 + 1) as usize
            * (self.zr.1 - self.zr.0 + 1) as usize
    }
}

fn limit(input: Vec<(bool, Cuboid)>, min: i32, max: i32) -> Vec<(bool, Cuboid)> {
    let out_of_bounds = |(a, b): (i32, i32)| -> bool { a > max || b < min };
    let normalize = |(a, b): (i32, i32)| -> (i32, i32) {
        let start = std::cmp::max(a, min);
        let end = std::cmp::min(b, max);
        (start, end)
    };
    input
        .into_iter()
        .filter_map(|(on, Cuboid { xr, yr, zr })| {
            if out_of_bounds(xr) || out_of_bounds(yr) || out_of_bounds(zr) {
                None
            } else {
                Some((
                    on,
                    Cuboid {
                        xr: normalize(xr),
                        yr: normalize(yr),
                        zr: normalize(zr),
                    },
                ))
            }
        })
        .collect::<Vec<_>>()
}

fn input() -> Vec<(bool, Cuboid)> {
    raw_input()
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let on = parts.next().unwrap() == "on";
            let (xr, yr, zr) = {
                let coordinates = parts.next().unwrap();
                let coordinates = coordinates.replace(|c| "xyz=".contains(c), "");
                let coordinates = coordinates
                    .split(',')
                    .map(|s| {
                        let point = s
                            .split("..")
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
                        (point[0], point[1])
                    })
                    .collect::<Vec<_>>();
                (coordinates[0], coordinates[1], coordinates[2])
            };
            (on, Cuboid { xr, yr, zr })
        })
        .collect()
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
        assert_eq!(474140, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2758514936282235, part2());
    }
}
