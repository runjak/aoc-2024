use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "./inputs/08/input.txt";

type N = i32;
type Coordinate = (N, N);

#[derive(Debug)]
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

fn bounding_box(input: &Input) -> Coordinate {
    *input.keys().max().unwrap_or(&(0, 0))
}

fn get_antennas(input: &Input) -> Vec<Antenna> {}

fn first() -> Result<(), Box<dyn Error>> {
    let wanted = 0;
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

    const EXAMPLE_PATH: &str = "./inputs/08/example.txt";
}
