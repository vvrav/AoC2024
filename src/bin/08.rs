use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let lines: Vec<&str> = input.split('\n').filter(|l| l.len() > 0).collect();
    let height = lines.len() as i32;
    let width = if height > 0 { lines[0].len() as i32 } else { 0 };
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c.is_ascii_alphanumeric() {
                let pos = (i as i32, j as i32);
                if let Some(v) = antennas.get_mut(&c) {
                    v.push(pos);
                } else {
                    antennas.insert(c, vec![pos]);
                }
            }
        });
    });
    return (antennas, height, width);
}

fn get_antinodes1(
    antennas: HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, pos_list) in antennas {
        for (idx, pos1) in pos_list.iter().enumerate() {
            for pos2 in &pos_list[idx + 1..] {
                let a1 = (2 * pos1.0 - pos2.0, 2 * pos1.1 - pos2.1);
                let a2 = (2 * pos2.0 - pos1.0, 2 * pos2.1 - pos1.1);
                if a1.0 >= 0 && a1.0 < height && a1.1 >= 0 && a1.1 < width {
                    antinodes.insert(a1);
                }
                if a2.0 >= 0 && a2.0 < height && a2.1 >= 0 && a2.1 < width {
                    antinodes.insert(a2);
                }
            }
        }
    }
    antinodes
}

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, height, width) = parse_input(input);
    let antinodes = get_antinodes1(antennas, height, width);
    Some(antinodes.len())
}

fn get_antinodes2(
    antennas: HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, pos_list) in antennas {
        for (idx, pos1) in pos_list.iter().enumerate() {
            for pos2 in &pos_list[idx + 1..] {
                let mut a = pos1.clone();
                while a.0 >= 0 && a.0 < height && a.1 >= 0 && a.1 < width {
                    antinodes.insert(a);
                    a.0 += pos1.0 - pos2.0;
                    a.1 += pos1.1 - pos2.1;
                }
                let mut a = pos1.clone();
                while a.0 >= 0 && a.0 < height && a.1 >= 0 && a.1 < width {
                    antinodes.insert(a);
                    a.0 -= pos1.0 - pos2.0;
                    a.1 -= pos1.1 - pos2.1;
                }
            }
        }
    }
    antinodes
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas, height, width) = parse_input(input);
    let antinodes = get_antinodes2(antennas, height, width);
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (result, height, width) =
            parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.get(&'A'), Some(&vec![(5, 6), (8, 8), (9, 9)]));
        assert_eq!(result.get(&'a'), None);
        assert_eq!(height, 12);
        assert_eq!(width, 12);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
