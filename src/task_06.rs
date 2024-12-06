use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

const INPUT_PATH: &str = "./inputs/06/input.txt";

type N = i32;
type Coordinate = (N, N);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Field {
    Floor,
    Wall,
    Player { direction: Direction },
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
    const EXAMPLE_PATH: &str = "./inputs/06/example.txt";
}
