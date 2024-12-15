use std::{collections::HashSet, fmt::Display};

advent_of_code::solution!(6);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn coords(&self) -> (i32, i32) {
        match self {
            Self::East => (0, 1),
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }

    fn turn(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::North => "North",
            Self::East => "East",
            Self::South => "South",
            Self::West => "West",
        };
        write!(f, "{}", s)
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut map: Vec<Vec<char>> = Vec::new();
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .for_each(|l| map.push(l.chars().collect()));
    let mut start = None;
    for (i, l) in (0..).zip(map.iter()) {
        match l.iter().position(|c| *c == '^') {
            Some(j) => {
                start = Some((i, j as i32));
                break;
            }
            None => (),
        }
    }
    let start = start.expect("Start position not found");
    (map, start)
}

fn run(map: &Vec<Vec<char>>, start: &(i32, i32)) -> usize {
    let mut pos = start.clone();
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut dir = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);

    loop {
        let d = dir.coords();
        let n = (pos.0 + d.0, pos.1 + d.1);
        if n.0 < 0 || n.0 >= height || n.1 < 0 || n.1 >= width {
            break;
        }
        if map[n.0 as usize][n.1 as usize] == '#' {
            dir = dir.turn();
            continue;
        }
        pos = n;
        visited.insert(pos);
    }

    visited.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start) = parse_input(input);
    let count = run(&map, &start);
    Some(count)
}

fn try_run(map: &Vec<Vec<char>>, start: &(i32, i32)) -> bool {
    let mut pos = start.clone();
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut dir = Direction::North;
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();
    visited.insert((pos, dir));

    loop {
        let d = dir.coords();
        let n = (pos.0 + d.0, pos.1 + d.1);
        if n.0 < 0 || n.0 >= height || n.1 < 0 || n.1 >= width {
            return false
        }
        if map[n.0 as usize][n.1 as usize] == '#' {
            dir = dir.turn();
        } else {
            pos = n;
        }
        if !visited.insert((pos, dir)) {
            return true
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start) = parse_input(input);
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut tmap = map.clone();
            tmap[i][j] = '#';
            if try_run(&tmap, &start) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "..#\n#.#\n#^.\n";
        let res = parse_input(input);
        assert_eq!(res, (vec![vec!['.','.','#'], vec!['#', '.','#'], vec!['#', '^', '.']], (2,1)));
    }


    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
