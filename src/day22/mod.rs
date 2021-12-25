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
        let x_left = other.xr.0 >= self.xr.0 && other.xr.0 <= self.xr.1;
        let x_right = other.xr.1 >= self.xr.0 && other.xr.1 <= self.xr.1;
        let x_inside = other.xr.0 <= self.xr.1 && other.xr.1 >= self.xr.1;
        let y_left = other.yr.0 >= self.yr.0 && other.yr.0 <= self.yr.1;
        let y_right = other.yr.1 >= self.yr.0 && other.yr.1 <= self.yr.1;
        let y_inside = other.yr.0 <= self.yr.1 && other.yr.1 >= self.yr.1;
        let z_left = other.zr.0 >= self.zr.0 && other.zr.0 <= self.zr.1;
        let z_right = other.zr.1 >= self.zr.0 && other.zr.1 <= self.zr.1;
        let z_inside = other.zr.0 <= self.zr.1 && other.zr.1 >= self.zr.1;
        (x_left || x_right || x_inside)
            && (y_left || y_right || y_inside)
            && (z_left || z_right || z_inside)
    }

    fn contained_by(&self, other: Cuboid) -> bool {
        self.xr.0 >= other.xr.0
            && self.xr.0 <= other.xr.1
            && self.xr.1 >= other.xr.0
            && self.xr.1 <= other.xr.1
            && self.yr.0 >= other.yr.0
            && self.yr.0 <= other.yr.1
            && self.yr.1 >= other.yr.0
            && self.yr.1 <= other.yr.1
            && self.zr.0 >= other.zr.0
            && self.zr.0 <= other.zr.1
            && self.zr.1 >= other.zr.0
            && self.zr.1 <= other.zr.1
    }

    pub fn count_active(&self) -> usize {
        (self.xr.1 - self.xr.0 + 1) as usize
            * (self.yr.1 - self.yr.0 + 1) as usize
            * (self.zr.1 - self.zr.0 + 1) as usize
    }
}

fn limit(input: Vec<(bool, Cuboid)>, min: i32, max: i32) -> Vec<(bool, Cuboid)> {
    input
        .into_iter()
        .filter_map(
            |(
                on,
                Cuboid {
                    xr: (xs, xe),
                    yr: (ys, ye),
                    zr: (zs, ze),
                },
            )| {
                if xs > max || ys > max || zs > max || xe < min || ye < min || ze < min {
                    None
                } else {
                    let xs = if xs < min { min } else { xs };
                    let xe = if xe > max { max } else { xe };
                    let ys = if ys < min { min } else { ys };
                    let ye = if ye > max { max } else { ye };
                    let zs = if zs < min { min } else { zs };
                    let ze = if ze > max { max } else { ze };
                    Some((
                        on,
                        Cuboid {
                            xr: (xs, xe),
                            yr: (ys, ye),
                            zr: (zs, ze),
                        },
                    ))
                }
            },
        )
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
                let coordinates = coordinates.replace("x", "");
                let coordinates = coordinates.replace("y", "");
                let coordinates = coordinates.replace("z", "");
                let coordinates = coordinates.replace("=", "");
                let coordinates = coordinates
                    .split(',')
                    .map(|s| s.split("..").collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                (
                    (
                        coordinates[0][0].parse().unwrap(),
                        coordinates[0][1].parse().unwrap(),
                    ),
                    (
                        coordinates[1][0].parse().unwrap(),
                        coordinates[1][1].parse().unwrap(),
                    ),
                    (
                        coordinates[2][0].parse().unwrap(),
                        coordinates[2][1].parse().unwrap(),
                    ),
                )
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
