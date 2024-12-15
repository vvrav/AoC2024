use std::num::ParseIntError;

advent_of_code::solution!(2);

fn parse_lines(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    let rlines: Result<Vec<_>, _> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect();
    rlines
}

pub fn part_one(input: &str) -> Option<usize> {
    let rlines = parse_lines(input);

    let nb_safe: Option<usize> = rlines.ok().map(|lines| {
        lines
            .iter()
            .filter(|line| {
                let difs: Vec<_> = line.windows(2).map(|w| w[1] - w[0]).collect();
                let is_safe: bool =
                    difs.iter().all(|x| *x > 0 && *x < 4) || difs.iter().all(|x| *x < 0 && *x > -4);
                println!("{:?} is safe? {}", line, is_safe);
                is_safe
            })
            .count()
    });

    nb_safe
}

pub fn part_two(input: &str) -> Option<usize> {
    let rlines = parse_lines(input);

    let nb_safe: Option<usize> = rlines.ok().map(|lines| {
        lines
            .iter()
            .filter(|line| {
                for i in 0..(line.len()) {
                    let sline: Vec<_> = line
                        .iter()
                        .enumerate()
                        .filter_map(|(j, v)| if i == j { None } else { Some(*v) })
                        .collect();
                    let diffs = get_diffs(&&sline);
                    if sline.last() > sline.first() && diffs.iter().all(|x| *x > 0 && *x < 4) {
                        return true;
                    }
                    if sline.last() < sline.first() && diffs.iter().all(|x| *x > -4 && *x < 0) {
                        return true;
                    }
                    println!("nok {:?} ({:?})", sline, diffs);
                }
                false
            })
            .count()
    });

    nb_safe
}

fn get_diffs(line: &&Vec<i32>) -> Vec<i32> {
    let difs: Vec<_> = line.windows(2).map(|w| w[1] - w[0]).collect();
    difs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
