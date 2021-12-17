pub fn day17() {
    println!("Result 17-1: {}", part1());
    println!("Result 17-2: {}", part2());
}

pub fn part1() -> usize {
    let mut best = 0;
    let window = input();
    let x_candidates = get_x_candidates(window);
    'outer: for yv in window.max_y.abs()..window.min_y.abs() {
        let maximum = calculate_self_sum(yv);
        if maximum > best {
            let mut steps = (2 * yv) + 1;
            let mut y = 0;
            let mut additional_steps = 0;
            'inner: loop {
                additional_steps += 1;
                y += yv - additional_steps;
                match window.match_y(y) {
                    WindowMatch::Before => {}
                    WindowMatch::In => {
                        break 'inner;
                    }
                    WindowMatch::After => {
                        break 'outer;
                    }
                }
            }
            steps += additional_steps;
            for x in &x_candidates {
                if let WindowMatch::In = window.match_x(calculate_x_at_step(*x, steps)) {
                    best = maximum;
                }
            }
        }
    }

    best as usize
}

pub fn part2() -> usize {
    let window = input();
    let options: Vec<(i32, i32)> = get_x_candidates(window)
        .into_iter()
        .flat_map(|xv| {
            let mut velocities = vec![];
            (window.min_y..=window.min_y.abs()).for_each(|yv| {
                if lands_in_window(window, xv, yv) {
                    velocities.push((xv, yv));
                }
            });
            velocities
        })
        .collect::<Vec<_>>();
    options.len()
}

fn lands_in_window(window: Window, xv: i32, yv: i32) -> bool {
    let mut steps = 0;
    let mut x = 0;
    let mut y = 0;
    loop {
        x += std::cmp::max(0, xv - steps);
        y += yv - steps;
        let to_match = (window.match_x(x), window.match_y(y));
        match to_match {
            (WindowMatch::In, WindowMatch::In) => {
                return true;
            }
            (WindowMatch::After, _) | (_, WindowMatch::After) => {
                return false;
            }
            _ => {}
        }
        steps += 1;
    }
}

fn calculate_x_at_step(x: i32, step: i32) -> i32 {
    if step >= x {
        calculate_self_sum(x)
    } else {
        let full_x = calculate_self_sum(x);
        let deficit = calculate_self_sum(x - step);
        full_x - deficit
    }
}

fn calculate_self_sum(num: i32) -> i32 {
    if num & 1 == 1 {
        num * ((num + 1) / 2)
    } else {
        (num + 1) * (num / 2)
    }
}

fn get_x_candidates(window: Window) -> Vec<i32> {
    let mut candidates = vec![];
    'outer: for x in 1..=window.max_x {
        if calculate_self_sum(x) < window.min_x {
            continue;
        }
        let mut sum = x;
        'inner: for i in 1..=x {
            match window.match_x(sum) {
                WindowMatch::Before => {}
                WindowMatch::In => {
                    candidates.push(x);
                    break 'inner;
                }
                WindowMatch::After => {
                    if i == 1 {
                        break 'outer;
                    }
                }
            }
            sum += x - i;
        }
    }
    candidates
}

fn input() -> Window {
    let parts = raw_input()
        .matches(|c: char| c.is_numeric() || "-,.".contains(c))
        .collect::<String>()
        .replace("..", ",")
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    Window::new(parts[0], parts[1], parts[2], parts[3])
}

#[derive(Copy, Clone, Debug)]
struct Window {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Window {
    pub fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn match_x(&self, x: i32) -> WindowMatch {
        if x < self.min_x {
            WindowMatch::Before
        } else if x > self.max_x {
            WindowMatch::After
        } else {
            WindowMatch::In
        }
    }

    pub fn match_y(&self, y: i32) -> WindowMatch {
        if y < self.min_y {
            WindowMatch::After
        } else if y > self.max_y {
            WindowMatch::Before
        } else {
            WindowMatch::In
        }
    }
}

#[derive(Debug)]
enum WindowMatch {
    Before,
    In,
    After,
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
        assert_eq!(45, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(112, part2());
    }
}
