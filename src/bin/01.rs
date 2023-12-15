use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let input_as_lines = input.lines();

    let get_first_char_as_string = |line: &str| line.chars().next().unwrap().to_string();
    let get_last_char_as_string = |line: &str| line.chars().last().unwrap().to_string();

    let result = input_as_lines
        .map(|line| line
             .chars()
             .fold(String::new(), |state, current| {
                 let maybe = current.to_string().parse::<u32>();

                 if let Ok(parsed) = maybe {
                     let parsed_string = parsed.to_string();
                     let state_string = state.to_string();

                     format!("{state_string}{parsed_string}")
                 } else {
                     state
                 }
             }))
    .map(|digits| {
        let first = get_first_char_as_string(&digits);
        let last = get_last_char_as_string(&digits);
        format!("{first}{last}")
    })
    .map(|digits| digits.parse::<u32>().unwrap())
    .sum();

    Some(result)
}

struct DigitStringEntry {
    string: String,
    digit: u32,
    regex: Regex,
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_as_lines = input.lines();

    let digit_string_vector = vec![
        DigitStringEntry {
            string: "oneight".to_string(),
            digit: 18,
            regex: Regex::new(r"\W*(oneight)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "twone".to_string(),
            digit: 21,
            regex: Regex::new(r"\W*(twone)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "threeight".to_string(),
            digit: 38,
            regex: Regex::new(r"\W*(threeight)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "fiveight".to_string(),
            digit: 58,
            regex: Regex::new(r"\W*(fiveight)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "sevenine".to_string(),
            digit: 79,
            regex: Regex::new(r"\W*(sevenine)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "eightwo".to_string(),
            digit: 82,
            regex: Regex::new(r"\W*(eightwo)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "eighthree".to_string(),
            digit: 83,
            regex: Regex::new(r"\W*(eighthree)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "nineight".to_string(),
            digit: 98,
            regex: Regex::new(r"\W*(nineight)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "one".to_string(),
            digit: 1,
            regex: Regex::new(r"\W*(one)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "two".to_string(),
            digit: 2,
            regex: Regex::new(r"\W*(two)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "three".to_string(),
            digit: 3,
            regex: Regex::new(r"\W*(three)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "four".to_string(),
            digit: 4,
            regex: Regex::new(r"\W*(four)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "five".to_string(),
            digit: 5,
            regex: Regex::new(r"\W*(five)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "six".to_string(),
            digit: 6,
            regex: Regex::new(r"\W*(six)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "seven".to_string(),
            digit: 7,
            regex: Regex::new(r"\W*(seven)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "eight".to_string(),
            digit: 8,
            regex: Regex::new(r"\W*(eight)\W*").unwrap(),
        },
        DigitStringEntry {
            string: "nine".to_string(),
            digit: 9,
            regex: Regex::new(r"\W*(nine)\W*").unwrap(),
        },
    ];

    let get_first_char_as_string = |line: &str| line.chars().next().unwrap().to_string();
    let get_last_char_as_string = |line: &str| line.chars().last().unwrap().to_string();

    fn replace_string_with_digit(line: &str, digit_string_vector: &Vec<DigitStringEntry>) -> String {
        let mut result = line.to_string();

        for entry in digit_string_vector {
            result = entry.regex.replace_all(&result, entry.digit.to_string().as_str()).to_string();
        }

        result
    }


    let result = input_as_lines
        .map(|line| replace_string_with_digit(line, &digit_string_vector))
        .map(|line| line
             .chars()
             .fold(String::new(), |state, current| {
                 let maybe = current.to_string().parse::<u32>();

                 if let Ok(parsed) = maybe {
                     let parsed_string = parsed.to_string();
                     let state_string = state.to_string();

                     format!("{state_string}{parsed_string}")
                 } else {
                     state
                 }
             }))
        .map(|digits| {
            let first = get_first_char_as_string(&digits);
            let last = get_last_char_as_string(&digits);
            format!("{first}{last}")
        })
        .map(|digits| digits.parse::<u32>().unwrap())
        .sum();

    Some(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
