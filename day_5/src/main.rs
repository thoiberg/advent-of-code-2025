use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let (fresh_ranges, ingredient_ids) = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(fresh_ranges, ingredient_ids);
    println!("Part One answer is {part_one_answer}"); // 865
}

fn part_one_solution(fresh_ranges: Vec<RangeInclusive<u64>>, ingredient_ids: Vec<u64>) -> u64 {
    ingredient_ids.iter().fold(0, |acc, ingredient_id| {
        let is_fresh = fresh_ranges
            .iter()
            .any(|range| range.contains(ingredient_id));
        if is_fresh { acc + 1 } else { acc }
    })
}

fn process_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    //
    let (fresh_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();

    let ranges = fresh_ranges
        .split("\n")
        .map(|fresh_range| {
            let (start, end) = fresh_range.split_once("-").unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            RangeInclusive::new(start, end)
        })
        .collect();

    let ids = ingredient_ids
        .split("\n")
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    (ranges, ids)
}

#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use super::*;

    fn test_data() -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
        let example_input = read_to_string("./example.txt").unwrap();

        process_input(&example_input)
    }

    #[test]
    fn test_process_input() {
        let (ranges, ingredient_ids) = test_data();

        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0].start(), &3);
        assert_eq!(ranges[0].end(), &5);

        assert_eq!(ingredient_ids.len(), 6);
        assert_eq!(ingredient_ids[0], 1);
    }

    #[test]
    fn test_part_one_solution_example() {
        let (ranges, ingredient_ids) = test_data();

        let part_one_answer = part_one_solution(ranges, ingredient_ids);

        assert_eq!(part_one_answer, 3);
    }
}
