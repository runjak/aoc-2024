use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "./inputs/05/input.txt";

type Page = i32;

#[derive(Debug, PartialEq)]
struct PageOrdering {
    before: Page,
    after: Page,
}

type Update = Vec<Page>;
type Input = (Vec<PageOrdering>, Vec<Update>);

fn read_input(path: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let parts = input.split("\n\n");

    let [page_orderings, updates] = parts.into_iter().collect::<Vec<_>>()[..] else {
        return Err("Stuff is broken yo.".into());
    };

    // FIXME continue.
    panic!("Soon to be done.");
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
    const EXAMPLE_PATH: &str = "./inputs/05/example.txt";

    #[test]
    fn should_calculate_first_example() {
        assert_eq!(1, 0);
    }

    #[test]
    fn should_calculate_second_example() {
        assert_eq!(1, 0);
    }
}
