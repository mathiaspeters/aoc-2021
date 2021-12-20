pub fn day20() {
    println!("Result 20-1: {}", part1());
    println!("Result 20-2: {}", part2());
}

pub fn part1() -> usize {
    process(2)
}

pub fn part2() -> usize {
    process(50)
}

fn process(iterations: usize) -> usize {
    let (lookup, mut image) = input((iterations as f64 * 1.5) as usize);
    image.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        );
    });
    for i in 0..iterations {
        let max_row = image.len() - 1;
        let max_col = image[0].len() - 1;
        let mut output = vec![vec![false; max_col + 1]; max_row + 1];
        for row in 0..image.len() {
            for col in 0..image[0].len() {
                let bits = get_adjacent((row, col), max_row, max_col).map(|p| match p {
                    Some((row, col)) => {
                        if image[row][col] {
                            1_u8
                        } else {
                            0_u8
                        }
                    }
                    None => 0_u8,
                });
                let index = binary_to_int(&bits);
                output[row][col] = lookup[index];
            }
        }
        if lookup[0] {
            output[0][0] = i & 1 == 0;
        }
        image = output;
    }
    println!("");
    image.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        );
    });
    image
        .iter()
        .map(|row| row.iter().filter(|p| **p).count())
        .sum()
}

fn get_adjacent(
    point: (usize, usize),
    max_row: usize,
    max_col: usize,
) -> [Option<(usize, usize)>; 9] {
    let (row, col) = point;
    let mut points = [None; 9];
    let row_sub = row.checked_sub(1);
    let row_add = if row >= max_row { None } else { Some(row + 1) };
    let col_sub = col.checked_sub(1);
    let col_add = if col >= max_col { None } else { Some(col + 1) };
    if row_sub.is_some() {
        if col_sub.is_some() {
            points[0] = Some((row_sub.unwrap(), col_sub.unwrap()));
        }
        points[1] = Some((row_sub.unwrap(), col));
        if col_add.is_some() {
            points[2] = Some((row_sub.unwrap(), col_add.unwrap()));
        }
    }
    if col_sub.is_some() {
        points[3] = Some((row, col_sub.unwrap()));
    }
    points[4] = Some((row, col));
    if col_add.is_some() {
        points[5] = Some((row, col_add.unwrap()));
    }
    if row_add.is_some() {
        if col_sub.is_some() {
            points[6] = Some((row_add.unwrap(), col_sub.unwrap()));
        }
        points[7] = Some((row_add.unwrap(), col));
        if col_add.is_some() {
            points[8] = Some((row_add.unwrap(), col_add.unwrap()));
        }
    }
    points
}

fn binary_to_int(bits: &[u8]) -> usize {
    let mut result = 0;
    let mut multiplier = 1;
    bits.iter().rev().for_each(|b| {
        result += multiplier * *b as usize;
        multiplier *= 2;
    });
    result
}

fn input(padding: usize) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut lines = raw_input().lines();
    let lookup = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let mut image = vec![vec![]; padding];
    lines.for_each(|line| {
        let mut row = vec![];
        for _ in 0..padding {
            row.push(false);
        }
        line.chars().for_each(|c| {
            row.push(c == '#');
        });
        for _ in 0..padding {
            row.push(false);
        }
        image.push(row);
    });
    for i in 0..padding {
        image[i] = vec![false; image[padding].len()];
        image.push(vec![false; image[padding].len()]);
    }
    (lookup, image)
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
        assert_eq!(35, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3351, part2());
    }
}
