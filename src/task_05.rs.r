use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "./inputs/05/input.txt";

type Page = i32;

#[derive(Debug, PartialEq)]
struct PageOrdering{
  before:Page,after:Page;
}

type Update=Vec<Page>
type Input = (Vec<PageOrdering>,Vec<Update>);

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
    use super::{find_x_mas, find_xmas, read_input};

    const EXAMPLE_PATH: &str = "./inputs/05/example.txt";

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
