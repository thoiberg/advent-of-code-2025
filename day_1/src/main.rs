use std::fs::read_to_string;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let puzzle_data = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(&puzzle_data);

    println!("The Part One Answer is {part_one_answer}"); // 964

    let part_two_answer = part_two_solution(&puzzle_data);
    println!("The Part Two Answer is {part_two_answer}"); // 5872
}

fn part_one_solution(instructions: &[Instruction]) -> usize {
    let mut current_position: i32 = 50;

    let positions: Vec<i32> = instructions
        .iter()
        .map(|instruction| match instruction.direction {
            Direction::Left => {
                let new_pos = (current_position - instruction.steps) % 100;

                current_position = if new_pos < 0 { 100 + new_pos } else { new_pos };
                current_position
            }
            Direction::Right => {
                current_position = (current_position + instruction.steps) % 100;
                current_position
            }
        })
        .collect();

    positions.into_iter().filter(|pos| pos == &0).count()
}

fn part_two_solution(instructions: &[Instruction]) -> i32 {
    let mut current_position: i32 = 50;

    let crossing_zeros: Vec<i32> = instructions
        .iter()
        .map(|instruction| match instruction.direction {
            Direction::Left => {
                let mut crossing_zero = instruction.steps / 100;
                let new_pos = current_position - (instruction.steps % 100);

                if current_position != 0 && new_pos <= 0 {
                    crossing_zero += 1;
                }

                current_position = if new_pos < 0 { 100 + new_pos } else { new_pos };

                crossing_zero
            }
            Direction::Right => {
                let mut crossing_zero = instruction.steps / 100;
                let new_pos = current_position + (instruction.steps % 100);

                if current_position != 0 && new_pos >= 100 {
                    crossing_zero += 1;
                }
                current_position = new_pos % 100;

                crossing_zero
            }
        })
        .collect();

    crossing_zeros.into_iter().sum()
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

fn process_input(contents: &str) -> Vec<Instruction> {
    contents
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            let direction = match chars[0] {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("expected L or R, got: {}", chars[0]),
            };

            let steps = (chars[1..].iter().collect::<String>())
                .parse::<i32>()
                .unwrap();

            Instruction { direction, steps }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Instruction> {
        let test_data = read_to_string("./example.txt").unwrap();

        process_input(&test_data)
    }

    #[test]
    fn test_process_input() {
        let data = test_data();

        assert_eq!(data.len(), 10);
        assert_eq!(data[0].direction, Direction::Left);
        assert_eq!(data[0].steps, 68);

        assert_eq!(data[2].direction, Direction::Right);
        assert_eq!(data[2].steps, 48);
    }

    #[test]
    fn test_part_one_example() {
        let test_data = test_data();

        assert_eq!(part_one_solution(&test_data), 3);
    }

    #[test]
    fn test_part_two_example() {
        let test_data = test_data();

        assert_eq!(part_two_solution(&test_data), 6);
    }

    #[test]
    fn test_reddit_examples() {
        let left_then_right = process_input("L50\nR50");
        assert_eq!(part_two_solution(&left_then_right), 1);

        let left_then_left = process_input("L50\nL50");
        assert_eq!(part_two_solution(&left_then_left), 1);

        let right_then_left = process_input("R50\nL50");
        assert_eq!(part_two_solution(&right_then_left), 1);

        let right_then_right = process_input("R50\nR50");
        assert_eq!(part_two_solution(&right_then_right), 1);
    }

    #[test]
    fn test_more_reddit_examples() {
        let left_then_left = process_input("L150\nL50");
        assert_eq!(part_two_solution(&left_then_left), 2);

        let left_then_right = process_input("L150\nR50");
        assert_eq!(part_two_solution(&left_then_right), 2);

        let right_then_left = process_input("R150\nL50");
        assert_eq!(part_two_solution(&right_then_left), 2);

        let right_then_right = process_input("R150\nR50");
        assert_eq!(part_two_solution(&right_then_right), 2);
    }

    #[test]
    fn test_replicating_mismatches() {
        // L855 from 85, first need to move from 50
        let left_mismatch = process_input("R35\nL855");
        assert_eq!(part_two_solution(&left_mismatch), 8);

        // R825 from 14, first need to move from 50
        let right_mismatch = process_input("L36\nR825");
        assert_eq!(part_two_solution(&right_mismatch), 8);
    }
}
