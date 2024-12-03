use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

// Seems like I just need to parse
// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day3.txt").unwrap();
    let sol1: i64 = part1(&input);
    let sol2: i64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &str) -> i64 {
    let mut result = 0;
    let mut input_iter = input.chars().peekable();

    while let Some(c) = input_iter.next() {
        match c {
            'm' => {
                if input_iter.next_if(|&c| c == 'u').is_some()
                    && input_iter.next_if(|&c| c == 'l').is_some()
                    && input_iter.next_if(|&c| c == '(').is_some()
                {
                    let mut num1 = String::new();
                    while let Some(d) = input_iter.next_if(|c| c.is_digit(10)) {
                        num1.push(d);
                    }

                    if input_iter.next_if(|&c| c == ',').is_some() {
                        let mut num2 = String::new();
                        while let Some(d) = input_iter.next_if(|c| c.is_digit(10)) {
                            num2.push(d);
                        }

                        if input_iter.next_if(|&c| c == ')').is_some() {
                            if let (Ok(n1), Ok(n2)) = (num1.parse::<i64>(), num2.parse::<i64>()) {
                                result += n1 * n2;
                            }
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    result
}

#[derive(Debug, Eq, PartialEq)]
enum MulSwitch {
    Do,
    Dont,
}

fn part2(input: &str) -> i64 {
    let mut result: i64 = 0;
    let mut input_iter = input.chars().peekable();
    let mut mul_switch = MulSwitch::Do;

    while let Some(c) = input_iter.next() {
        match c {
            'm' => {
                if input_iter.next_if(|&c| c == 'u').is_some()
                    && input_iter.next_if(|&c| c == 'l').is_some()
                    && input_iter.next_if(|&c| c == '(').is_some()
                {
                    let mut num1 = String::new();
                    while let Some(d) = input_iter.next_if(|c| c.is_digit(10)) {
                        num1.push(d);
                    }

                    if input_iter.next() == Some(',') {
                        let mut num2 = String::new();
                        while let Some(d) = input_iter.next_if(|c| c.is_digit(10)) {
                            num2.push(d);
                        }

                        if input_iter.next_if(|&c| c == ')').is_some() {
                            if let (Ok(n1), Ok(n2)) = (num1.parse::<i64>(), num2.parse::<i64>()) {
                                if mul_switch == MulSwitch::Do {
                                    result += n1 * n2;
                                }
                            }
                        }
                    }
                }
            }
            'd' => {
                if input_iter.next_if(|&c| c == 'o').is_some() {
                    mul_switch = MulSwitch::Do;
                    if input_iter.next_if(|&c| c == 'n').is_some()
                        && input_iter.next_if(|&c| c == '\'').is_some()
                        && input_iter.next_if(|&c| c == 't').is_some()
                    {
                        mul_switch = MulSwitch::Dont;
                    }
                }
            }
            _ => continue,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        let input = "xmmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_take_while() {
        let s = "123,456";
        let mut chars = s.chars();
        let nums: String = chars.by_ref().take_while(|c| c.is_digit(10)).collect();
        assert_eq!(nums, "123");
        assert_eq!(chars.next(), Some('4'));
    }
}
