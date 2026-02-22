use std::fs::read_to_string;

use ndarray::Array2;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let floor_map = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(&floor_map); // 1433
    println!("Part One answer is {part_one_answer}");

    let part_two_answer = part_two_solution(floor_map.clone());
    println!("Part Two answer is {part_two_answer}");
}

fn part_one_solution(floor_map: &Array2<Position>) -> usize {
    positions_with_enough_space(floor_map).len()
}

fn part_two_solution(mut floor_map: Array2<Position>) -> usize {
    let mut movable_rolls = positions_with_enough_space(&floor_map);
    let mut total_rolls_removed = movable_rolls.len();

    while !movable_rolls.is_empty() {
        movable_rolls.into_iter().for_each(|(y, x)| {
            floor_map[[y, x]] = Position::Empty;
        });

        movable_rolls = positions_with_enough_space(&floor_map);
        total_rolls_removed += movable_rolls.len();
    }

    total_rolls_removed
}

fn positions_with_enough_space(floor_map: &Array2<Position>) -> Vec<(usize, usize)> {
    floor_map
        .indexed_iter()
        .filter(|(_, pos)| pos == &&Position::PaperRoll)
        .filter_map(|((y, x), _)| {
            let surrounding_positions: Vec<_> = get_surrounding_positions(y, x)
                .into_iter()
                .filter_map(|new_coord| floor_map.get(new_coord))
                .collect();

            let paper_roll_tiles = surrounding_positions.into_iter().fold(0, |acc, pos| {
                if pos == &Position::PaperRoll {
                    acc + 1
                } else {
                    acc
                }
            });

            if paper_roll_tiles < 4 {
                Some((y, x))
            } else {
                None
            }
        })
        .collect()
}

fn get_surrounding_positions(y: usize, x: usize) -> Vec<(usize, usize)> {
    let new_coords = [
        // top
        (y.checked_sub(1), x.checked_sub(1)),
        (y.checked_sub(1), Some(x)),
        (y.checked_sub(1), x.checked_add(1)),
        // middle
        (Some(y), x.checked_sub(1)),
        (Some(y), x.checked_add(1)),
        // bottom
        (y.checked_add(1), x.checked_sub(1)),
        (y.checked_add(1), Some(x)),
        (y.checked_add(1), x.checked_add(1)),
    ];

    new_coords
        .into_iter()
        .filter_map(|coord| {
            if let (Some(y), Some(x)) = coord {
                Some((y, x))
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Position {
    Empty,
    PaperRoll,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::PaperRoll,
            x => panic!("Expected either . or @, got: {}", x),
        }
    }
}

fn process_input(input: &str) -> ndarray::Array2<Position> {
    let positions: Vec<Vec<Position>> = input
        .lines()
        .map(|line| line.chars().map(|position| position.into()).collect())
        .collect();

    let rows = positions.len();
    let columns = positions[0].len();

    let positions = positions.into_iter().flatten().collect();

    Array2::from_shape_vec((rows, columns), positions).unwrap()
}

#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use super::*;

    fn test_data() -> Array2<Position> {
        let example_data = read_to_string("./example.txt").unwrap();
        process_input(&example_data)
    }

    #[test]
    fn test_process_input() {
        let example_data = read_to_string("./example.txt").unwrap();
        let floor_map = process_input(&example_data);

        assert_eq!(floor_map.rows().into_iter().len(), 10);
        assert_eq!(floor_map.columns().into_iter().len(), 10);
    }

    #[test]
    fn test_part_one_solution_example() {
        let test_data = test_data();

        let valid_tiles = part_one_solution(&test_data);

        assert_eq!(valid_tiles, 13);
    }

    #[test]
    fn test_part_two_solution_example() {
        let moved_rolls = part_two_solution(test_data());

        assert_eq!(moved_rolls, 43);
    }
}
