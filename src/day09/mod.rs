pub fn day9() {
    println!("Result  9-1: {}", part1());
    println!("Result  9-2: {}", part2());
}

pub fn part1() -> usize {
    let map = build_map();
    let low_points = get_low_points(&map);
    low_points
        .iter()
        .map(|(row, col)| map[*row][*col] as usize + 1)
        .sum()
}

pub fn part2() -> usize {
    let map = build_map();
    let basins = get_low_points(&map)
        .into_iter()
        .map(|point| get_basin(&map, point))
        .collect::<Vec<_>>();
    let mut basin_sizes = basins.iter().map(|basin| basin.len()).collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    let mut result = 1;
    basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .for_each(|size| result *= size);
    result
}

fn get_basin(map: &Vec<Vec<u8>>, low_point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut basin = vec![low_point];
    check_point(map, &mut basin, low_point);
    basin
}

fn check_point(map: &Vec<Vec<u8>>, basin: &mut Vec<(usize, usize)>, point: (usize, usize)) {
    let adjacent = get_adjacent(map, point);
    let mut to_check = vec![];
    adjacent.into_iter().for_each(|point| {
        if !basin.contains(&point) && map[point.0][point.1] < 9 {
            to_check.push(point);
            basin.push(point);
        }
    });
    to_check
        .into_iter()
        .for_each(|point| check_point(map, basin, point));
}

fn build_map() -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = vec![];
    raw_input().split('\n').for_each(|line| {
        let mut row = vec![];
        line.chars().for_each(|c| match c {
            '0' => row.push(0),
            '1' => row.push(1),
            '2' => row.push(2),
            '3' => row.push(3),
            '4' => row.push(4),
            '5' => row.push(5),
            '6' => row.push(6),
            '7' => row.push(7),
            '8' => row.push(8),
            '9' => row.push(9),
            _ => {}
        });
        map.push(row);
    });
    map
}

fn get_low_points(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let min = get_adjacent(map, (row, col))
                .into_iter()
                .fold(true, |acc, (x, y)| acc && map[row][col] < map[x][y]);
            if min {
                result.push((row, col));
            }
        }
    }
    result
}

fn get_adjacent(map: &Vec<Vec<u8>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut output = vec![];
    if point.0 > 0 {
        output.push((point.0 - 1, point.1));
    }
    if point.0 < map.len() - 1 {
        output.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        output.push((point.0, point.1 - 1));
    }
    if point.1 < map[0].len() - 1 {
        output.push((point.0, point.1 + 1));
    }
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
        assert_eq!(15, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1134, part2());
    }
}
