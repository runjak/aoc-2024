use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

const INPUT_PATH: &str = "./inputs/09/input.txt";

type N = i32;

#[derive(PartialEq, Debug)]
struct File {
    id: N,
    length: N,
}

#[derive(PartialEq, Debug)]
enum Segment {
    Space(N),
    File(File),
}

type Input = Vec<Segment>;

fn read_input(path: &str) -> Result<Input, Box<dyn Error>> {
    let mut input = fs::read_to_string(path)?;

    Ok(Vec::new())
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = 0;
    println!("{}", wanted);
    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
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
    const EXAMPLE_PATH: &str = "./inputs/09/example.txt";
}
