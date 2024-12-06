use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "./inputs/06/input.txt";

type N = i32;
type Coordinate = (N, N);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction_to_delta(direction: &Direction) -> Coordinate {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn add_coordinates(a: &Coordinate, b: &Coordinate) -> Coordinate {
    (a.0 + b.0, a.1 + b.1)
}

enum Field {
    Floor,
    Wall,
    Player { direction: Direction },
    Visited,
}

type Input = HashMap<Coordinate, Field>;

fn read_input(path: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Coordinate, Field)> {
            line.chars()
                .enumerate()
                .map(|(x, field)| -> (Coordinate, Field) {
                    let field = match field {
                        '#' => Field::Wall,
                        '^' => Field::Player {
                            direction: Direction::Up,
                        },
                        _ => Field::Floor,
                    };

                    ((x as N, y as N), field)
                })
                .collect()
        })
        .collect())
}

fn find_guard(input: &Input) -> Option<Coordinate> {
    input
        .iter()
        .find(|(_, field)| -> bool {
            if let Field::Player { direction: _ } = field {
                return true;
            }

            false
        })
        .map(|(coordinate, _)| *coordinate)
}

fn rotate_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn walk_guard(mut input: Input) -> Input {
    let Some(mut position) = find_guard(&input) else {
        return input;
    };

    while let Some(Field::Player { direction }) = input.get(&position) {
        let mut next_position = add_coordinates(&position, &direction_to_delta(direction));
        let mut next_direction = *direction;

        if let Some(Field::Wall) = input.get(&next_position) {
            next_direction = rotate_right(direction);
            next_position = add_coordinates(&position, &direction_to_delta(&next_direction));
        }

        input.insert(position, Field::Visited);
        if input.contains_key(&next_position) {
            input.insert(
                next_position,
                Field::Player {
                    direction: next_direction,
                },
            );
        }
        position = next_position;
    }

    return input;
}

fn solution_1(input: Input) -> N {
    let walked_input = walk_guard(input);

    walked_input
        .into_iter()
        .filter(|(_, field)| match field {
            Field::Visited => true,
            _ => false,
        })
        .count() as N
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = solution_1(input);
    println!("{}", wanted);
    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    let wanted = 0;
    println!("{}", wanted);
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("01-1:");
    first()?;
    println!("01-2:");
    second()
}

#[cfg(test)]
mod tests {
    use super::{read_input, solution_1};

    const EXAMPLE_PATH: &str = "./inputs/06/example.txt";

    #[test]
    fn should_compute_example_1() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_1(input);
        let expected = 41;

        assert_eq!(actual, expected);
    }
}
