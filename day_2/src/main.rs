use std::{fs::read_to_string, ops::RangeInclusive};

use fancy_regex::Regex;

fn main() {
    let puzzle_data = read_to_string("./puzzle_input.txt").unwrap();
    let id_ranges = process_input(&puzzle_data);

    let part_one_answer = part_one_solution(&mut id_ranges.clone());
    println!("Part One answer is: {part_one_answer}"); // 12,586,854,255

    let part_two_answer = part_two_solution(&mut id_ranges.clone());
    println!("Part Two answer is: {part_two_answer}"); // 17,298,174,201

    // let part_two_answer_regex = part_two_solution_regex(&mut id_ranges.clone());
    // println!("Part Two answer is: {part_two_answer_regex}"); // 17,298,174,201
}

fn part_one_solution(id_ranges: &mut [RangeInclusive<u64>]) -> u64 {
    id_ranges.iter_mut().fold(0, |acc, id_range| {
        let mut range_acc = 0;
        for i in id_range {
            let number_str = i.to_string();
            let (first_half, second_half) = number_str.split_at(number_str.len() / 2);

            if first_half == second_half {
                range_acc += i;
            }
        }

        acc + range_acc
    })
}

fn part_two_solution(id_ranges: &mut [RangeInclusive<u64>]) -> u64 {
    // let expr = Regex::new(r"^(\d+)\1+$").unwrap();

    id_ranges.iter_mut().fold(0, |acc, id_range| {
        let mut range_acc = 0;
        for id in id_range {
            // let number_str = id.to_string();
            let number_chars: Vec<char> = id.to_string().chars().collect();

            // if expr.is_match(&number_str).unwrap() {
            //     range_acc += id
            // }
            let number_char_length = number_chars.len();

            for chunk_size in 1..=(number_char_length / 2) {
                if !number_char_length.is_multiple_of(chunk_size) {
                    continue;
                }

                let chunks: Vec<&[char]> = number_chars.chunks(chunk_size).collect();
                let first_chunk = chunks[0];
                let have_pattern = chunks.into_iter().all(|chunk| chunk == first_chunk);

                if have_pattern {
                    range_acc += id;
                    break;
                }
            }
        }

        acc + range_acc
    })
}

#[allow(dead_code)]
fn part_two_solution_regex(id_ranges: &mut [RangeInclusive<u64>]) -> u64 {
    let expr = Regex::new(r"^(\d+)\1+$").unwrap();

    id_ranges.iter_mut().fold(0, |acc, id_range| {
        let mut range_acc = 0;
        for id in id_range {
            let number_str = id.to_string();

            if expr.is_match(&number_str).unwrap() {
                range_acc += id
            }
        }

        acc + range_acc
    })
}

fn process_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .flat_map(|line| {
            line.split(',')
                .map(|id_range| {
                    let (start, end) = id_range.split_once('-').unwrap();

                    RangeInclusive::new(start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use super::*;

    fn test_data() -> Vec<RangeInclusive<u64>> {
        let input_data = read_to_string("./example.txt").unwrap();

        process_input(&input_data)
    }

    #[test]
    fn test_process_input() {
        let input_data = read_to_string("./example.txt").unwrap();

        let ids = process_input(&input_data);

        assert_eq!(ids.len(), 11);
        assert_eq!(ids[0].start(), &11);
        assert_eq!(ids[0].end(), &22);
    }

    #[test]
    fn test_part_one_example() {
        let mut example_data = test_data();

        assert_eq!(part_one_solution(&mut example_data), 1227775554);
    }

    #[test]
    fn test_part_two_example() {
        let mut example_data = test_data();

        assert_eq!(part_two_solution(&mut example_data), 4174379265);
    }
}
