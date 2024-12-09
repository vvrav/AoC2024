use std::collections::HashMap;

advent_of_code::solution!(1);

fn lists_of_input(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let nums: Result<Vec<_>, _> = input
        .split(char::is_whitespace)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>())
        .collect();
    nums.ok()
        .map(|v| v.chunks_exact(2).map(|c| (c[0], c[1])).unzip())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = lists_of_input(input)?;
    a.sort();
    b.sort();
    let zip: Vec<_> = a.iter().zip(b.iter()).collect();
    Some(zip.iter().fold(
        0,
        |acc: u32, (a, b)| if a > b { acc + *a - *b } else { acc + *b - *a },
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut a, mut b) = lists_of_input(input)?;
    a.sort();
    b.sort();
    let m: HashMap<u32, u32> = b.chunk_by(|a, b| a == b)
        .map(|e| (e[0], e.len() as u32)).collect();

    let res = a.iter().fold(0u32, |acc, xa|
        acc + m.get(xa).unwrap_or(&0) * *xa
    );
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lists_of_input() {
        let result = lists_of_input("3   4\n4 3  \n2   5\n");
        assert_eq!(result, Some((vec![3, 4, 2], vec![4, 3, 5])));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
