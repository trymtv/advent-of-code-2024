use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| line.split_once(&" ".repeat(3)).unwrap())
        .map(|pair| {
            (
                pair.0.parse::<u32>().unwrap(),
                pair.1.parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = parse_input(input);
    list1.sort();
    list2.sort();

    let diff: Vec<u32> = list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| a.abs_diff(b))
        .collect();

    Some(diff.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_input(input);
    let list1_counts = list1.into_iter().counts();
    let list2_counts = list2.into_iter().counts();

    let total = list1_counts.keys().fold(0, |acc, key| {
        acc + *key as usize
            * list1_counts.get(key).unwrap_or(&0_usize)
            * list2_counts.get(key).unwrap_or(&0_usize)
    });

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

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
