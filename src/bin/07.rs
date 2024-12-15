advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let lines = input.split('\n').filter(|l| !l.is_empty());

    lines
        .map(|l| {
            let (target_str, operands_str) = l.split_once(':').unwrap();
            let operands: Vec<_> = operands_str
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            let target = target_str.parse::<u64>().unwrap();
            (target, operands)
        })
        .collect()
}

fn test_aux(target: &u64, operands: &[u64], acc: u64) -> bool {
    if operands.is_empty() {
        return *target == acc;
    }
    if acc > *target {
        return false;
    }
    let head = operands[0];
    let tail = &operands[1..];

    test_aux(target, tail, acc + head) || test_aux(target, tail, acc * head)
}

fn test(target: &u64, operands: &[u64]) -> bool {
    !operands.is_empty() && test_aux(target, &operands[1..], operands[0])
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let res: u64 = equations
        .iter()
        .filter(|(t, o)| test(t, o))
        .map(|(t, _)| *t)
        .sum();
    Some(res)
}

fn testp2_aux(target: &u64, operands: &[u64], acc: u64) -> bool {
    if operands.is_empty() {
        return *target == acc;
    }
    if acc > *target {
        return false;
    }
    let head = operands[0];
    let tail = &operands[1..];

    testp2_aux(target, tail, acc + head)
        || testp2_aux(target, tail, acc * head)
        || testp2_aux(
            target,
            tail,
            format!("{}{}", acc, head).parse::<u64>().unwrap(),
        )
}

fn testp2(target: &u64, operands: &[u64]) -> bool {
    !operands.is_empty() && testp2_aux(target, &operands[1..], operands[0])
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let res: u64 = equations
        .iter()
        .filter(|(t, o)| testp2(t, o))
        .map(|(t, _)| *t)
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let res = parse_input("21037: 9 7 18 13\n192: 17 8 14\n");
        assert_eq!(
            res,
            vec![(21037, vec![9, 7, 18, 13]), (192, vec![17, 8, 14])]
        );
    }

    #[test]
    fn test_test() {
        assert!(!test(&123, &[10, 20, 30]));
        assert!(test(&3267, &[81, 40, 27]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
