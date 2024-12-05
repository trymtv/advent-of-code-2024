advent_of_code::solution!(3);

fn sum_segment(input: &str) -> u32 {
    input
        .split("mul(")
        .map(|segment| {
            let end = match segment.find(")") {
                Some(v) => v,
                None => {
                    return 0;
                }
            };
            let args: Vec<&str> = segment[..end].split(",").collect();
            if args.len() != 2 {
                return 0;
            }
            let arg1 = match args[0].parse::<i32>() {
                Ok(v) => v,
                _ => {
                    return 0;
                }
            };
            let arg2 = match args[1].parse::<i32>() {
                Ok(v) => v,
                _ => {
                    return 0;
                }
            };
            (arg1 * arg2) as u32
        })
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = sum_segment(input);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let joined_input = input.split_whitespace().collect::<Vec<&str>>().join("");
    let total = joined_input
        .split("do()")
        .map(|segment| sum_segment(segment.split("don't()").collect::<Vec<&str>>()[0]))
        .sum::<u32>();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        println!("test 1");
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
