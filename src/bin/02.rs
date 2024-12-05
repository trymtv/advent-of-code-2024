advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(report: &[i32]) -> bool {
    let sign = (report[0] - report[1]).signum();
    for (i, number) in report.iter().take(report.len() - 1).enumerate() {
        let res = *number - report[i + 1];
        if (sign != res.signum()) || (res.abs() > 3) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = parse(input);
    let number_of_valid: Vec<_> = numbers
        .into_iter()
        .filter(|report| is_safe_report(report))
        .collect();
    Some(number_of_valid.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = parse(input);
    let number_of_valid: Vec<_> = numbers
        .iter()
        .filter(|report| {
            let sign = (report[0] - report[1]).signum();
            for (i, number) in report.iter().take(report.len() - 1).enumerate() {
                let res = *number - report[i + 1];
                if (sign != res.signum()) || res.abs() > 3 {
                    let mut skip_1 = report[..i].to_vec();
                    skip_1.extend_from_slice(&report[i + 1..]);
                    let mut skip_2 = report[..(i + 1)].to_vec();
                    skip_2.extend_from_slice(&report[(i + 2)..]);
                    if is_safe_report(&skip_1) || is_safe_report(&skip_2) {
                        return true;
                    }
                    return false;
                }
            }
            true
        })
        .collect();
    Some(number_of_valid.len() as u32)
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
