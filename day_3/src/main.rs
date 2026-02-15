use std::fs::read_to_string;

type Voltages = Vec<Vec<u32>>;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let voltages = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(&voltages);
    println!("Part One answer is: {part_one_answer}"); // 17,092
}

fn part_one_solution(voltages: &Voltages) -> u32 {
    voltages
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

fn process_input(input: &str) -> Voltages {
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

    fn test_input() -> Voltages {
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
}
