use std::{fs::read_to_string, str::FromStr};

use ndarray::Array2;

fn main() {
    let puzzle_input = read_to_string("./puzzle_input.txt").unwrap();
    let floor_map = process_input(&puzzle_input);

    let part_one_answer = part_one_solution(&floor_map);
    println!("Part One answer is {part_one_answer}");
}

fn part_one_solution(floor_map: &Array2<Position>) -> u32 {
    // get the coords for the (at most) 8 surrounding elements
    // get the values of the elements
    // sum them if they are Position::PaperRoll
    // if greater than 4 then return
    // for ((y, x), position) in floor_map.indexed_iter() {
    //     if position == &Position::Empty {
    //         continue;
    //     }

    // }

    floor_map
        .indexed_iter()
        .filter(|(_, pos)| pos == &&Position::PaperRoll)
        .fold(0, |acc, ((y, x), _)| {
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

            if paper_roll_tiles < 4 { acc + 1 } else { acc }
        })
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

#[derive(Debug, PartialEq, Eq)]
enum Position {
    Empty,
    PaperRoll,
}

impl TryFrom<char> for Position {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '@' => Ok(Self::PaperRoll),
            _ => Err(()),
        }
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "@" => Ok(Self::PaperRoll),
            _ => Err(()),
        }
    }
}

fn process_input(input: &str) -> ndarray::Array2<Position> {
    let boop: Vec<Vec<Position>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|position| position.try_into().unwrap())
                .collect()
        })
        .collect();

    let rows = boop.len();
    let columns = boop[0].len();

    let boop = boop.into_iter().flatten().collect();

    Array2::from_shape_vec((rows, columns), boop).unwrap()
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
}
