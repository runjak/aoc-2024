use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

const INPUT_PATH: &str = "./inputs/08/input.txt";

type N = i32;
type Coordinate = (N, N);

#[derive(Clone, Copy, Debug)]
struct Antenna {
    location: Coordinate,
    frequency: char,
}

#[derive(Debug)]
enum Field {
    Empty,
    Antenna(Antenna),
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
                .map(|(x, frequency)| -> (Coordinate, Field) {
                    let location: Coordinate = (x as N, y as N);

                    if frequency == '.' {
                        return (location, Field::Empty);
                    }

                    (
                        location,
                        Field::Antenna(Antenna {
                            frequency,
                            location,
                        }),
                    )
                })
                .collect()
        })
        .collect())
}

fn get_bounding_box(input: &Input) -> Coordinate {
    *input.keys().max().unwrap_or(&(0, 0))
}

fn in_bounding_box(location: &Coordinate, bounding_box: &Coordinate) -> bool {
    if location.0 < 0 || location.1 < 0 {
        return false;
    }

    if location.0 > bounding_box.0 || location.1 > bounding_box.1 {
        return false;
    }

    true
}

fn get_antennas(input: Input) -> Vec<Antenna> {
    input
        .into_values()
        .filter_map(|field| match field {
            Field::Empty => None,
            Field::Antenna(antenna) => Some(antenna),
        })
        .collect()
}

fn group_by_frequency(antennas: Vec<Antenna>) -> HashMap<char, Vec<Antenna>> {
    let mut antennas_by_frequency: HashMap<char, Vec<Antenna>> = HashMap::new();

    for antenna in antennas.into_iter() {
        match antennas_by_frequency.get_mut(&antenna.frequency) {
            Some(existing) => {
                existing.push(antenna);
            }
            None => {
                antennas_by_frequency.insert(antenna.frequency, Vec::from([antenna]));
            }
        }
    }

    antennas_by_frequency
}

fn antinode_pair(a: &Antenna, b: &Antenna) -> Vec<Coordinate> {
    let (ax, ay) = a.location;
    let (bx, by) = b.location;
    let (dx, dy) = (bx - ax, by - ay);

    Vec::from([(ax - dx, ay - dy), (bx + dx, by + dy)])
}

fn get_antinodes_by_pairs(antennas: &Vec<Antenna>) -> HashSet<Coordinate> {
    let mut antinodes = HashSet::new();

    for (index, first_antenna) in antennas.iter().enumerate() {
        for second_antenna in antennas[index + 1..].iter() {
            for antinode in antinode_pair(first_antenna, second_antenna) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes
}

fn solution_1(input: Input) -> N {
    let bounding_box = get_bounding_box(&input);
    let grouped_antennas = group_by_frequency(get_antennas(input));

    let antinodes = grouped_antennas
        .values()
        .flat_map(|antennas| get_antinodes_by_pairs(antennas))
        .filter(|location| in_bounding_box(location, &bounding_box))
        .collect::<HashSet<_>>();

    antinodes.len() as N
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = solution_1(input);
    println!("{}", wanted);
    Ok(())
}

fn antinodes_in_bounding_box(
    a: &Antenna,
    b: &Antenna,
    bounding_box: &Coordinate,
) -> HashSet<Coordinate> {
    let (ax, ay) = a.location;
    let (bx, by) = b.location;

    let start_and_direction = [
        (a.location, (ax - bx, ay - by)),
        (b.location, (bx - ax, by - ay)),
    ];

    start_and_direction
        .into_iter()
        .flat_map(|(start, direction)| -> Vec<Coordinate> {
            let mut antinodes = Vec::new();

            let ((sx, sy), (dx, dy)) = (start, direction);

            let mut factor = 0;
            loop {
                let location = (sx + factor * dx, sy + factor * dy);

                if !in_bounding_box(&location, bounding_box) {
                    break;
                }

                antinodes.push(location);
                factor += 1;
            }

            antinodes
        })
        .collect()
}

fn get_antinodes_by_bounding_box(
    antennas: &Vec<Antenna>,
    bounding_box: &Coordinate,
) -> HashSet<Coordinate> {
    let mut antinodes = HashSet::new();

    for (index, first_antenna) in antennas.iter().enumerate() {
        for second_antenna in antennas[index + 1..].iter() {
            for antinode in
                antinodes_in_bounding_box(first_antenna, second_antenna, bounding_box).into_iter()
            {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes
}

fn solution_2(input: Input) -> N {
    let bounding_box = get_bounding_box(&input);
    let grouped_antennas = group_by_frequency(get_antennas(input));

    let antinodes = grouped_antennas
        .values()
        .flat_map(|antennas| get_antinodes_by_bounding_box(antennas, &bounding_box))
        .collect::<HashSet<_>>();

    antinodes.len() as N
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
    use super::{read_input, solution_1, solution_2};

    const EXAMPLE_PATH: &str = "./inputs/08/example.txt";

    #[test]
    fn should_compute_first_example() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_1(input);
        let expected = 14;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_compute_second_example() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_2(input);
        let expected = 34;

        assert_eq!(actual, expected);
    }
}
