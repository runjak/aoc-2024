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

type TurnLog = Vec<Coordinate>;

fn walk_guard(mut input: Input) -> (Input, TurnLog) {
    let mut turn_log = Vec::new();

    let Some(mut position) = find_guard(&input) else {
        return (input, turn_log);
    };

    while let Some(Field::Player { direction }) = input.get(&position) {
        let mut next_position = add_coordinates(&position, &direction_to_delta(direction));
        let mut next_direction = *direction;

        if let Some(Field::Wall) = input.get(&next_position) {
            next_direction = rotate_right(direction);
            next_position = add_coordinates(&position, &direction_to_delta(&next_direction));
            turn_log.push(position);
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

    return (input, turn_log);
}

fn solution_1(input: Input) -> N {
    let (walked_input, _) = walk_guard(input);

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

type Triplet = (Coordinate, Coordinate, Coordinate);

fn into_triplets(turn_log: TurnLog) -> Vec<Triplet> {
    turn_log
        .iter()
        .zip(turn_log[1..].iter())
        .zip(turn_log[2..].iter())
        .map(|((a, b), c)| (*a, *b, *c))
        .collect()
}

fn keep_same((ax, ay): &Coordinate, (bx, by): &Coordinate) -> Coordinate {
    let x = if ax == bx { *ax } else { 0 };
    let y = if ay == by { *ay } else { 0 };

    (x, y)
}

fn scale_coordinate(s: N, (x, y): &Coordinate) -> Coordinate {
    (s * x, s * y)
}

fn suspect_coordinate((a, b, c): &Triplet) -> Coordinate {
    let first = add_coordinates(a, &scale_coordinate(-1, &keep_same(a, b)));
    let second = add_coordinates(c, &scale_coordinate(-1, &keep_same(b, c)));

    add_coordinates(&first, &second)
}

fn coordinate_range(from: &Coordinate, to: &Coordinate) -> Vec<Coordinate> {
    let mut range = Vec::new();

    for x in from.0..(to.0 + 1) {
        for y in from.1..(to.1 + 1) {
            range.push((x, y));
        }
    }

    range
}

fn can_loop(input: &Input, triplet: &Triplet) -> bool {
    let suspect = suspect_coordinate(triplet);
    let wall_candidates = coordinate_range(&triplet.2, &suspect);

    wall_candidates
        .into_iter()
        .map(|coordinate| input.get(&coordinate).unwrap_or(&Field::Wall))
        .all(|field| match field {
            Field::Wall => false,
            _ => true,
        })
}

fn solution_2(input: Input) -> N {
    let (walked_input, turn_log) = walk_guard(input);
    let triplets = into_triplets(turn_log);

    triplets
        .into_iter()
        .filter(|triplet| can_loop(&walked_input, triplet))
        .count() as N
}

fn second() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = solution_2(input);
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
    use super::{read_input, solution_1, solution_2, suspect_coordinate};

    const EXAMPLE_PATH: &str = "./inputs/06/example.txt";

    #[test]
    fn should_compute_example_1() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_1(input);
        let expected = 41;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_suspect_the_right_coordinate() {
        let actual = suspect_coordinate(&((4, 1), (8, 1), (8, 6)));
        let expected = (4, 6);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_compute_example_2() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_2(input);
        let expected = 6;

        assert_eq!(actual, expected);
    }
}
