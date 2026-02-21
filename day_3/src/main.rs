use std::fs::read_to_string;

type BatteryBanks = Vec<Vec<u64>>;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let battery_banks = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(&battery_banks);
    println!("Part One answer is: {part_one_answer}"); // 17,092

    let part_two_answer = part_two_solution(&battery_banks);
    println!("Part Two answer is: {part_two_answer}"); // 170,147,128,753,455
}

fn part_one_solution(battery_banks: &BatteryBanks) -> u32 {
    battery_banks.iter().fold(0, |acc, batteries| {
        let max_jolts = max_jolt(batteries, None, 2, vec![]);
        acc + (convert_vec_to_num(&max_jolts) as u32)
    })
}

fn part_two_solution(battery_banks: &BatteryBanks) -> u64 {
    battery_banks.iter().fold(0, |acc, batteries| {
        let max_jolts = max_jolt(batteries, None, 12, vec![]);
        acc + convert_vec_to_num(&max_jolts)
    })
}

fn max_jolt(
    batteries: &[u64],
    lower_bound: Option<usize>,
    digit_to_find: usize,
    max_jolts: Vec<u64>,
) -> Vec<u64> {
    if digit_to_find == 0 {
        return max_jolts;
    };

    let lower_bound = lower_bound.unwrap_or(0);

    let upper_bound = batteries.len() - digit_to_find;
    let valid_digits = &batteries[lower_bound..=upper_bound];

    let max = valid_digits.iter().max().unwrap();
    // This _should_be ok because if there are multiple numbers we always want to take the one to the left
    let max_pos = valid_digits.iter().position(|x| x == max).unwrap();
    let next_max_bound = lower_bound + max_pos + 1;

    let new_max_jolts = max_jolts.into_iter().chain([*max]).collect();

    max_jolt(
        batteries,
        Some(next_max_bound),
        digit_to_find - 1,
        new_max_jolts,
    )
}

fn convert_vec_to_num(digits: &[u64]) -> u64 {
    digits.iter().rev().enumerate().fold(0, |acc, (i, volt)| {
        acc + *volt * 10u64.pow(i.try_into().unwrap())
    })
}

fn process_input(input: &str) -> BatteryBanks {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .map(|digit| digit.into())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use super::*;

    fn test_input() -> BatteryBanks {
        let example_text = read_to_string("./example.txt").unwrap();

        process_input(&example_text)
    }

    #[test]
    fn test_process_input() {
        let voltages = test_input();

        assert_eq!(voltages.len(), 4);
        assert_eq!(
            voltages[0],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_part_one_solution_example() {
        let voltages = test_input();

        assert_eq!(part_one_solution(&voltages), 357);
    }

    #[test]
    fn test_part_two_max_jolt() {
        let battery_bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];

        let max_joltage = max_jolt(&battery_bank, None, 12, vec![]);

        assert_eq!(max_joltage, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]);

        let battery_bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];

        let max_joltage = max_jolt(&battery_bank, None, 12, vec![]);

        assert_eq!(max_joltage, vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]);

        let battery_bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];

        let max_joltage = max_jolt(&battery_bank, None, 12, vec![]);

        assert_eq!(max_joltage, vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]);

        let battery_bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];

        let max_joltage = max_jolt(&battery_bank, None, 12, vec![]);

        assert_eq!(max_joltage, vec![8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
    }

    #[test]
    fn test_part_two_solution_example() {
        let voltages = test_input();

        assert_eq!(part_two_solution(&voltages), 3_121_910_778_619);
    }
}
