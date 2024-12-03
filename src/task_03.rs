use std::{error::Error, fs};

use regex::Regex;

fn read_input(path: String) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

type N = i32;

#[derive(Debug, PartialEq)]
struct Mul {
    a: N,
    b: N,
}

fn match_muls(input: String) -> Vec<Mul> {
    let mul_regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let captures = mul_regex.captures_iter(&input);

    captures
        .filter_map(|capture| -> Option<Mul> {
            let a = capture.name("a")?.as_str().parse::<N>().ok()?;
            let b = capture.name("b")?.as_str().parse::<N>().ok()?;

            Some(Mul { a, b })
        })
        .collect()
}

fn solution_1(path: String) -> Result<N, Box<dyn Error>> {
    let input = read_input(path)?;
    let muls = match_muls(input);

    Ok(muls.iter().map(|Mul { a, b }| -> N { (*a) * (*b) }).sum())
}

fn first() -> Result<(), Box<dyn Error>> {
    let wanted = solution_1("./inputs/03/input.txt".to_owned())?;
    println!("{}", wanted);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul { a: N, b: N },
}

fn match_instructions(input: String) -> Vec<Instruction> {
    let instruction_regex =
        Regex::new(r"((?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<a>\d+),(?<b>\d+)\)))")
            .unwrap();

    let captures = instruction_regex.captures_iter(&input);

    captures
        .filter_map(|capture| -> Option<Instruction> {
            let i_do = capture.name("do");
            let i_dont = capture.name("dont");
            let i_mul = capture.name("mul");

            match (i_do, i_dont, i_mul) {
                (Some(_), _, _) => Some(Instruction::Do),
                (None, Some(_), _) => Some(Instruction::Dont),
                (None, None, _) => {
                    let a = capture.name("a")?.as_str().parse::<N>().ok()?;
                    let b = capture.name("b")?.as_str().parse::<N>().ok()?;

                    Some(Instruction::Mul { a, b })
                }
            }
        })
        .collect()
}

fn filter_instructions(instructions: Vec<Instruction>) -> Vec<Mul> {
    let mut result = Vec::new();
    let mut do_enabled = true;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Do => {
                do_enabled = true;
            }
            Instruction::Dont => {
                do_enabled = false;
            }
            Instruction::Mul { a, b } => {
                if do_enabled {
                    result.push(Mul { a: *a, b: *b });
                }
            }
        }
    }

    return result;
}

fn solution_2(path: String) -> Result<N, Box<dyn Error>> {
    let input = read_input(path)?;
    let muls = filter_instructions(match_instructions(input));

    Ok(muls.iter().map(|Mul { a, b }| -> N { (*a) * (*b) }).sum())
}

fn second() -> Result<(), Box<dyn Error>> {
    let wanted = solution_2("./inputs/03/input.txt".to_owned())?;
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
    use super::{
        filter_instructions, match_instructions, match_muls, read_input, solution_1, solution_2,
        Instruction, Mul, N,
    };

    #[test]
    fn finds_example_muls() {
        let input = read_input("./inputs/03/example.txt".to_owned()).unwrap();
        let actual = match_muls(input);

        let expected = Vec::from([
            Mul { a: 2, b: 4 },
            Mul { a: 5, b: 5 },
            Mul { a: 11, b: 8 },
            Mul { a: 8, b: 5 },
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn calculates_first_example() {
        let actual = solution_1("./inputs/03/example.txt".to_owned()).unwrap();
        let expected: N = 161;

        assert_eq!(actual, expected);
    }

    #[test]
    fn finds_instructions() {
        let input = read_input("./inputs/03/example2.txt".to_owned()).unwrap();
        let actual = match_instructions(input);

        let expected = Vec::from([
            Instruction::Mul { a: 2, b: 4 },
            Instruction::Dont,
            Instruction::Mul { a: 5, b: 5 },
            Instruction::Mul { a: 11, b: 8 },
            Instruction::Do,
            Instruction::Mul { a: 8, b: 5 },
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn finds_enabled_muls() {
        let input = read_input("./inputs/03/example2.txt".to_owned()).unwrap();
        let actual = filter_instructions(match_instructions(input));

        let expected = Vec::from([Mul { a: 2, b: 4 }, Mul { a: 8, b: 5 }]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn calculates_second_example() {
        let actual = solution_2("./inputs/03/example2.txt".to_owned()).unwrap();
        let expected: N = 48;

        assert_eq!(actual, expected);
    }
}
