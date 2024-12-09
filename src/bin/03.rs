use regex::Regex;

advent_of_code::solution!(3);


fn cleanup(input: String) -> String {
    let p = input.find("don't()");
    if p.is_none() {
        return input
    }
    let (p1, p2) = input.split_at(p.unwrap());
    let p = p2.find("do()");
    if p.is_none() {
        return p1.to_string()
    }
    let (_, p3) = p2.split_at(p.unwrap());
    return cleanup(p1.to_string() + p3)
}

fn get_mults(input: &str) -> Vec<u32> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let res: Vec<u32> = re
        .captures_iter(input)
        .map(|c| {
            //  println!("reading {}", c[0].to_string())           ;
            c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap()
        })
        .collect();
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let products = get_mults(input);
    // println!("{:?}", products);
    Some(products.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cleaned_input = cleanup(input.to_string());
    println!("cleaned string:\n{}\n", cleaned_input);
    part_one(&cleaned_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_cleanup() {
        let result = cleanup(advent_of_code::template::read_file_part("examples", DAY, 2).to_string());
        assert_eq!(result, "xmul(2,4)&mul[3,7]!^do()?mul(8,5))".to_string());
    }


    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
