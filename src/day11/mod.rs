pub fn day11() {
    println!("Result 11-1: {}", part1());
    println!("Result 11-2: {}", part2());
}

pub fn part1() -> usize {
    let mut flashes = 0;
    let mut board = board();
    let mut set_to_zero = vec![];
    let mut flashed = vec![];
    for _ in 0..100 {
        for row in 0..10 {
            for col in 0..10 {
                if board[row][col] < 9 {
                    board[row][col] += 1;
                } else if board[row][col] == 9 {
                    board[row][col] = 10;
                    set_to_zero.push((row, col));
                    flashed.push((row, col));
                }
            }
        }
        let mut iteration_flashes = 0;
        while !flashed.is_empty() {
            iteration_flashes += flashed.len();
            let mut new_flashed = vec![];
            flashed.iter().for_each(|point| {
                let adjacent = get_adjacent(*point);
                adjacent.into_iter().for_each(|(row, col)| {
                    if board[row][col] < 9 {
                        board[row][col] += 1;
                    } else if board[row][col] == 9 {
                        board[row][col] = 10;
                        set_to_zero.push((row, col));
                        new_flashed.push((row, col));
                    }
                });
            });
            flashed = new_flashed;
        }
        set_to_zero.iter().for_each(|(r, c)| board[*r][*c] = 0);
        set_to_zero.clear();
        flashes += iteration_flashes;
    }
    flashes
}

pub fn part2() -> usize {
    let mut board = board();
    let mut set_to_zero = vec![];
    let mut flashed = vec![];
    let mut i = 0;
    loop {
        i += 1;
        for row in 0..10 {
            for col in 0..10 {
                if board[row][col] < 9 {
                    board[row][col] += 1;
                } else if board[row][col] == 9 {
                    board[row][col] = 10;
                    set_to_zero.push((row, col));
                    flashed.push((row, col));
                }
            }
        }
        while !flashed.is_empty() {
            let mut new_flashed = vec![];
            flashed.iter().for_each(|point| {
                let adjacent = get_adjacent(*point);
                adjacent.into_iter().for_each(|(row, col)| {
                    if board[row][col] < 9 {
                        board[row][col] += 1;
                    } else if board[row][col] == 9 {
                        board[row][col] = 10;
                        set_to_zero.push((row, col));
                        new_flashed.push((row, col));
                    }
                });
            });
            flashed = new_flashed;
        }
        if set_to_zero.len() == 100 {
            return i;
        }
        set_to_zero.iter().for_each(|(r, c)| board[*r][*c] = 0);
        set_to_zero.clear();
    }
}

fn get_adjacent(point: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = point;
    let mut points = vec![];
    if row > 0 && col > 0 {
        points.extend_from_slice(&[(row - 1, col - 1), (row - 1, col), (row, col - 1)]);
    } else if row > 0 {
        points.push((row - 1, col));
    } else if col > 0 {
        points.push((row, col - 1));
    }
    if row < 9 && col < 9 {
        points.extend_from_slice(&[(row + 1, col + 1), (row + 1, col), (row, col + 1)]);
    } else if row < 9 {
        points.push((row + 1, col));
    } else if col < 9 {
        points.push((row, col + 1));
    }
    if row > 0 && col < 9 {
        points.push((row - 1, col + 1));
    }
    if row < 9 && col > 0 {
        points.push((row + 1, col - 1));
    }
    points
}

fn board() -> [[u8; 10]; 10] {
    let mut output = [[0; 10]; 10];
    raw_input().lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            output[row][col] = super::util::char_to_u8(c).unwrap();
        });
    });
    output
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
        assert_eq!(1656, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part2());
    }
}
