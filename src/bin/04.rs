advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let mut count: u32 = 0;
    let height = lines.len();
    let width = lines[0].len();
    // rows
    for r in &lines {
        count += r.matches("XMAS").count() as u32;
        count += r.matches("SAMX").count() as u32;
    }
    // cols
    for i in 0..width {
        let s: String = (0..height)
            .filter_map(|j| lines[j].chars().nth(i))
            .collect();
        count += s.matches("XMAS").count() as u32;
        count += s.matches("SAMX").count() as u32;
    }
    // diags
    for i in 0..height {
        let s: String = (0..height - i)
            .filter_map(|j| lines[i + j].chars().nth(j))
            .collect();
        count += s.matches("XMAS").count() as u32;
        count += s.matches("SAMX").count() as u32;
        let s: String = (0..height - i)
            .filter_map(|j| lines[height - i - j - 1].chars().nth(j))
            .collect();
        count += s.matches("XMAS").count() as u32;
        count += s.matches("SAMX").count() as u32;
    }
    for j in 1..width {
        let s: String = (0..width - j)
            .filter_map(|i| lines[i].chars().nth(i + j))
            .collect();
        count += s.matches("XMAS").count() as u32;
        count += s.matches("SAMX").count() as u32;
        let s: String = (0..width - j)
            .filter_map(|i: usize| lines[(width - 1) - i].chars().nth(i + j))
            .collect();
        count += s.matches("XMAS").count() as u32;
        count += s.matches("SAMX").count() as u32;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut m1: Vec<(usize, usize)> = vec![];
    let mut m2: Vec<(usize, usize)> = vec![];

    for i in 0..height {
        let s: String = (0..height - i)
            .filter_map(|j| lines[i + j].chars().nth(j))
            .collect();

        s.match_indices("MAS")
            .map(|(j, _)| (i + j, j))
            .for_each(|e| m1.push(e));
        s.match_indices("SAM")
            .map(|(j, _)| (i + j, j))
            .for_each(|e| m1.push(e));

        let s: String = (0..height - i)
            .filter_map(|j| lines[height - i - j - 1].chars().nth(j))
            .collect();

        s.match_indices("MAS")
            .map(|(j, _)| (height - i - j - 1, j))
            .for_each(|e| m2.push(e));
        s.match_indices("SAM")
            .map(|(j, _)| (height - i - j - 1, j))
            .for_each(|e| m2.push(e));
    }
    for j in 1..width {
        let s: String = (0..width - j)
            .filter_map(|i| lines[i].chars().nth(i + j))
            .collect();

        s.match_indices("MAS")
            .map(|(i, _)| (i, i + j))
            .for_each(|e| m1.push(e));
        s.match_indices("SAM")
            .map(|(i, _)| (i, i + j))
            .for_each(|e| m1.push(e));

        let s: String = (0..width - j)
            .filter_map(|i: usize| lines[(width - 1) - i].chars().nth(i + j))
            .collect();

        s.match_indices("MAS")
            .map(|(i, _)| ((width - 1) - i, i + j))
            .for_each(|e| m2.push(e));
        s.match_indices("SAM")
            .map(|(i, _)| ((width - 1) - i, i + j))
            .for_each(|e| m2.push(e));
    }

    let c = m1
        .iter()
        .filter(|(x1, y1)| m2.contains(&(*x1 + 2, *y1)))
        .count() as u32;

    Some(c)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
