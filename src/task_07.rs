use std::{cmp::max, error::Error, fs};

const INPUT_PATH: &str = "./inputs/07/input.txt";

type N = i64;

#[derive(Debug, PartialEq)]
struct Equation {
    value: N,
    inputs: Vec<N>,
}

type Input = Vec<Equation>;

fn read_input(path: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .filter_map(|line| -> Option<Equation> {
            let [value, inputs] = line.split(": ").collect::<Vec<_>>()[..] else {
                return None;
            };

            let value = value.parse::<N>().ok()?;
            let inputs = inputs
                .split(" ")
                .filter_map(|input| -> Option<N> { input.parse().ok() })
                .collect();

            Some(Equation { value, inputs })
        })
        .collect())
}

#[derive(PartialEq, Debug)]
enum Operator {
    Add,
    Multiply,
}

fn next_operator(operators: N) -> (Operator, N) {
    let operator = if (operators & 0x1) == 0 {
        Operator::Add
    } else {
        Operator::Multiply
    };
    let next_operators = operators >> 1;

    (operator, next_operators)
}

fn calculate_equation(equation: &Equation, mut operators: N) -> N {
    let mut inputs = equation.inputs.to_owned();
    inputs.reverse();

    let mut value = inputs.pop().unwrap();

    while let Some(input) = inputs.pop() {
        let (operator, next_operators) = next_operator(operators);
        operators = next_operators;

        match operator {
            Operator::Add => value += input,
            Operator::Multiply => value *= input,
        }
    }

    value
}

fn operator_choices(equation: &Equation) -> N {
    let power: u32 = (equation.inputs.len() - 1).try_into().unwrap();
    max(2_i64.pow(power), 0)
}

fn equation_solvable(equation: &Equation) -> bool {
    for operators in 0..operator_choices(equation) {
        let value = calculate_equation(equation, operators);

        if equation.value == value {
            return true;
        }
    }

    false
}

fn solution_1(input: Input) -> N {
    input
        .into_iter()
        .filter(|equation| equation_solvable(equation))
        .map(|equation| equation.value)
        .sum::<N>()
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
    use super::{calculate_equation, next_operator, read_input, solution_1, Equation, Operator};

    const EXAMPLE_PATH: &str = "./inputs/07/example.txt";

    #[test]
    fn next_operator_behaves() {
        assert_eq!(next_operator(0), (Operator::Add, 0));
        assert_eq!(next_operator(1), (Operator::Multiply, 0));
        assert_eq!(next_operator(2), (Operator::Add, 1));
        assert_eq!(next_operator(3), (Operator::Multiply, 1));
    }

    #[test]
    fn calculates_equation_as_expected() {
        let equation = Equation {
            value: 3267,
            inputs: Vec::from([81, 40, 27]),
        };
        let actual = calculate_equation(&equation, 2);
        assert_eq!(actual, equation.value)
    }

    #[test]
    fn should_compute_example_1() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_1(input);
        let expected = 3749;

        assert_eq!(actual, expected);
    }
}
