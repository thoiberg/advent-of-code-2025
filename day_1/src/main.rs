use std::fs::read_to_string;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let puzzle_data = process_input(&puzzle_input);
    let part_one_answer = part_one_solution(&puzzle_data);

    println!("The Part One Answer is {part_one_answer}"); // 964
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

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
}

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
}
