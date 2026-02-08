use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let puzzle_data = read_to_string("./puzzle_input.txt").unwrap();
    let id_ranges = process_input(&puzzle_data);

    let part_one_answer = part_one_solution(&mut id_ranges.clone());
    println!("Part One answer is: {part_one_answer}"); // 12586854255
}

// will probably need to increase to a u64
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
    // for each range
    // for each number
    // convert to string, split on (char length / 2)
    // check if the first part matches the second part (probs do array comp here?)
    // if match then return
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
}
