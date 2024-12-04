use std::{collections::HashMap, error::Error, fs};

const EXAMPLE_PATH: &str = "./inputs/04/example.txt";
const INPUT_PATH: &str = "./inputs/04/input.txt";

type N = i32;
type Coordinate = (N, N);
type Input = HashMap<Coordinate, char>;

fn read_input(path: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Coordinate, char)> {
            line.chars()
                .enumerate()
                .map(|(x, c)| -> (Coordinate, char) { ((x as N, y as N), c) })
                .collect()
        })
        .collect::<Input>())
}

fn add(a: Coordinate, b: Coordinate) -> Coordinate {
    (a.0 + b.0, a.1 + b.1)
}

fn find_x_coordinates(input: Input) -> Vec<Coordinate> {
    input
        .iter()
        .filter_map(|(coordinate, char)| -> Option<Coordinate> {
            if *char == 'X' {
                return Some(*coordinate);
            }

            None
        })
        .collect()
}

fn x_deltass() -> Vec<Vec<Coordinate>> {
    Vec::from([
        Vec::from([(0, 1), (0, 2), (0, 3)]),
        Vec::from([(1, 1), (2, 2), (3, 3)]),
        Vec::from([(1, 0), (2, 0), (3, 0)]),
        Vec::from([(1, -1), (2, -2), (3, -3)]),
        Vec::from([(0, -1), (0, -2), (0, -3)]),
        Vec::from([(-1, -1), (-2, -2), (-3, -3)]),
        Vec::from([(-1, 0), (-2, 0), (-3, 0)]),
        Vec::from([(-1, 1), (-2, 2), (-3, 3)]),
    ])
}

fn check_xmas(input: &Input, coordinates: &Vec<Coordinate>) -> bool {
    let word = coordinates
        .iter()
        .map(|coordinate| -> char { *input.get(coordinate).unwrap_or(&'.') })
        .collect::<String>();

    word == "XMAS"
}

fn find_xmas(input: Input) -> Vec<Vec<Coordinate>> {
    let x_coordinates = find_x_coordinates(input.clone());

    let candidates = x_coordinates
        .into_iter()
        .flat_map(|start_coordinate| -> Vec<Vec<Coordinate>> {
            x_deltass()
                .into_iter()
                .map(|deltas| -> Vec<Coordinate> {
                    Vec::from([start_coordinate])
                        .into_iter()
                        .chain(
                            deltas
                                .into_iter()
                                .map(|delta| -> Coordinate { add(start_coordinate, delta) }),
                        )
                        .collect()
                })
                .collect()
        })
        .filter(|candidate| -> bool { check_xmas(&input, candidate) })
        .collect::<Vec<Vec<Coordinate>>>();

    return candidates;
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = find_xmas(input).len();
    println!("{}", wanted);
    Ok(())
}

fn find_a_coordinates(input: Input) -> Vec<Coordinate> {
    input
        .iter()
        .filter_map(|(coordinate, char)| -> Option<Coordinate> {
            if *char == 'A' {
                return Some(*coordinate);
            }

            None
        })
        .collect()
}

fn check_x_mas(input: &Input, a_coordinate: Coordinate) -> bool {
    let w1 = [
        *input.get(&add(a_coordinate, (-1, -1))).unwrap_or(&'.'),
        'A',
        *input.get(&add(a_coordinate, (1, 1))).unwrap_or(&'.'),
    ]
    .into_iter()
    .collect::<String>();

    let w2 = [
        *input.get(&add(a_coordinate, (-1, 1))).unwrap_or(&'.'),
        'A',
        *input.get(&add(a_coordinate, (1, -1))).unwrap_or(&'.'),
    ]
    .into_iter()
    .collect::<String>();

    (w1 == "MAS" || w1 == "SAM") && (w2 == "MAS" || w2 == "SAM")
}

fn find_x_mas(input: Input) -> Vec<Coordinate> {
    let a_coordinates = find_a_coordinates(input.clone());

    a_coordinates
        .into_iter()
        .filter(|a_coordinate| check_x_mas(&input, *a_coordinate))
        .collect()
}

fn second() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = find_x_mas(input).len();
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
    use super::{find_x_mas, find_xmas, read_input, EXAMPLE_PATH};

    #[test]
    fn should_calculate_first_example() {
        let input = read_input(EXAMPLE_PATH).unwrap();

        let actual = find_xmas(input).len();
        let expected = 18;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_second_example() {
        let input = read_input(EXAMPLE_PATH).unwrap();

        let actual = find_x_mas(input).len();
        let expected = 9;

        assert_eq!(actual, expected);
    }
}
