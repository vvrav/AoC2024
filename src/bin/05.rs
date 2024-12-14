use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

fn parse_order(lines: &Vec<&str>) -> HashMap<u32, Vec<u32>> {
    let mut m: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in lines {
        let (k, v) = line.split_once('|').unwrap();
        let key: u32 = k.parse().unwrap();
        let val: u32 = v.parse().unwrap();

        if let Some(v) = m.get_mut(&key) {
            v.push(val);
        } else {
            m.insert(key, vec![val]);
        }
    }

    m
}

fn is_smaller(a: u32, b: u32, ordering: &HashMap<u32, Vec<u32>>) -> bool {
    if let Some(av) = ordering.get(&a) {
        if av.contains(&b) {
            return true;
        }
    }
    false
}

fn sorter(a: u32, b: u32, ordering: &HashMap<u32, Vec<u32>>) -> Ordering {
    if is_smaller(a, b, ordering) {
        return Ordering::Less;
    }
    if is_smaller(b, a, ordering) {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn is_correct(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    !update.iter().enumerate().any(|(i, v)| {
        if let Some(r) = rules.get(v) {
            r.iter()
                .any(|rv| update.iter().position(|x| x == rv).is_some_and(|p| p < i))
        } else {
            false
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.split('\n').collect();
    let i = lines.iter().position(|l| l.len() == 0).unwrap();
    let (rules, rest) = lines.split_at(i);
    let rules: Vec<_> = rules.into();
    let updates: Vec<_> = rest
        .iter()
        .filter(|l| l.len() > 0)
        .map(|s| {
            s.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let orders = parse_order(&rules);
    let mut count = 0;
    for u in updates {
        if is_correct(&u, &orders) {
            count += u[u.len() / 2];
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.split('\n').collect();
    let i = lines.iter().position(|l| l.len() == 0).unwrap();
    let (rules, rest) = lines.split_at(i);
    let rules: Vec<_> = rules.into();
    let updates: Vec<_> = rest
        .iter()
        .filter(|l| l.len() > 0)
        .map(|s| {
            s.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let orders = parse_order(&rules);
    let mut count = 0;
    for u in updates {
        if !is_correct(&u, &orders) {
            let mut uc = u.clone();
            uc.sort_by(|a, b| sorter(*a, *b, &orders));
            count += uc[uc.len() / 2];
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_order() {
        let input_lines = vec!["1|2", "1|3", "2|5"];
        let res = parse_order(&input_lines);
        assert_eq!(res.get(&1), Some(&vec![2, 3]));
        assert_eq!(res.get(&2), Some(&vec![5]));
        assert_eq!(res.get(&3), None);
    }

    #[test]
    fn test_is_smaller() {
        let input_lines = vec!["1|2", "1|3", "2|5"];
        let res = parse_order(&input_lines);
        assert_eq!(is_smaller(1, 2, &res), true);
        assert_eq!(is_smaller(1, 5, &res), false);
        assert_eq!(is_smaller(1, 4, &res), false)
    }

    #[test]
    fn test_sorter() {
        let input_lines = vec!["1|2", "1|3", "2|5"];
        let res = parse_order(&input_lines);
        assert_eq!(sorter(1, 2, &res), Ordering::Less);
        assert_eq!(sorter(1, 5, &res), Ordering::Equal);
        assert_eq!(sorter(1, 4, &res), Ordering::Equal);
        assert_eq!(sorter(5, 2, &res), Ordering::Greater);
    }

    #[test]
    fn test_is_correct() {
        let input_lines = vec!["1|2", "1|3", "2|5"];
        let order = parse_order(&input_lines);
        assert_eq!(is_correct(&vec![1, 2, 3], &order), true);
        assert_eq!(is_correct(&vec![2, 1, 3], &order), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
