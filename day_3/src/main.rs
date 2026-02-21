use std::fs::read_to_string;

type BatteryBanks = Vec<Vec<u32>>;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let battery_banks = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(&battery_banks);
    println!("Part One answer is: {part_one_answer}"); // 17,092

    let part_two_answer = part_two_solution(&battery_banks);
    println!("Part Two answer is: {part_two_answer}"); // 170,147,128,753,455
}

fn part_one_solution(battery_banks: &BatteryBanks) -> u32 {
    battery_banks
        .iter()
        .fold(0, |acc, voltage| acc + max_joltage(voltage))
}

fn max_joltage(battery_bank: &[u32]) -> u32 {
    let mut second = battery_bank[battery_bank.len() - 1];
    let mut first = battery_bank[battery_bank.len() - 2];

    for battery in battery_bank.iter().rev().skip(2) {
        if *battery >= first {
            let new_first = [*battery, first].into_iter().max().unwrap();
            let new_second = [first, second].into_iter().max().unwrap();

            first = new_first;
            second = new_second;
        }
    }

    first * 10 + second
}

fn part_two_solution(battery_banks: &BatteryBanks) -> u64 {
    let mut total_voltage = 0;

    for batteries in battery_banks {
        let max_jolts = part_two_max_jolt(batteries, 0, 12, vec![]);
        let max_jolt_num = max_jolts
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, volt)| {
                acc + (*volt as u64) * 10u64.pow(i.try_into().unwrap())
            });

        total_voltage += max_jolt_num;
    }

    total_voltage
}

fn part_two_max_jolt(
    batteries: &[u32],
    max_bound: usize,
    digit_to_find: usize,
    mut max_jolts: Vec<u32>,
) -> Vec<u32> {
    if digit_to_find == 0 {
        return max_jolts;
    };

    let upper_bound = batteries.len() - digit_to_find;
    let lower_bound = max_bound;
    // find the next subslice and call
    let valid_digits = &batteries[lower_bound..=upper_bound];
    // get the position of the array
    let max = valid_digits.iter().max().unwrap();
    // This _should_be ok because if there are multiple numbers we always want to take the one to the left
    let max_pos = valid_digits.iter().position(|x| x == max).unwrap();
    let next_max_bound = max_bound + max_pos + 1;

    max_jolts.push(*max);

    part_two_max_jolt(batteries, next_max_bound, digit_to_find - 1, max_jolts)
}

fn process_input(input: &str) -> BatteryBanks {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
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
    fn test_max_voltage() {
        let voltage = vec![9, 9, 8];
        assert_eq!(max_joltage(&voltage), 99);
    }

    #[test]
    fn test_part_two_max_jolt() {
        let battery_bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];

        let max_jolt = part_two_max_jolt(&battery_bank, 0, 12, vec![]);

        assert_eq!(max_jolt, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]);

        let battery_bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];

        let max_jolt = part_two_max_jolt(&battery_bank, 0, 12, vec![]);

        assert_eq!(max_jolt, vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]);

        let battery_bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];

        let max_jolt = part_two_max_jolt(&battery_bank, 0, 12, vec![]);

        assert_eq!(max_jolt, vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]);

        let battery_bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];

        let max_jolt = part_two_max_jolt(&battery_bank, 0, 12, vec![]);

        assert_eq!(max_jolt, vec![8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
    }

    #[test]
    fn test_part_two_solution_example() {
        let voltages = test_input();

        assert_eq!(part_two_solution(&voltages), 3_121_910_778_619);
    }
}
