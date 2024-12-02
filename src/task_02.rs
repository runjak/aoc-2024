use std::{error::Error, fs};

type N = i32;
type Input = Vec<Vec<N>>;

fn read_input(path: String) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let lines = input.lines();

    Ok(lines
        .map(|line| -> Vec<N> {
            line.split(" ")
                .filter_map(|n| -> Option<N> { n.parse::<N>().ok() })
                .collect()
        })
        .collect())
}

#[derive(PartialEq, Debug)]
enum Direction {
    Unknown,
    Increments,
    Decrements,
}

fn report_is_safe(report: Vec<N>) -> bool {
    let firsts = report.iter();
    let seconds = report[1..].iter();

    let mut direction: Direction = Direction::Unknown;

    for (a, b) in firsts.zip(seconds) {
        match direction {
            Direction::Decrements => {
                if a < b {
                    return false;
                }
            }
            Direction::Increments => {
                if a > b {
                    return false;
                }
            }
            Direction::Unknown => {
                if a > b {
                    direction = Direction::Decrements;
                } else if b > a {
                    direction = Direction::Increments
                }
            }
        }

        let delta = (a - b).abs();
        if delta < 1 || delta > 3 {
            return false;
        }
    }

    return true;
}

fn solution_1(path: String) -> Result<usize, Box<dyn Error>> {
    let input = read_input(path)?;

    Ok(input
        .into_iter()
        .filter(|r| report_is_safe(r.to_owned()))
        .count())
}

fn first() -> Result<(), Box<dyn Error>> {
    let wanted = solution_1("./inputs/02/input.txt".to_owned())?;
    println!("{}", wanted);
    Ok(())
}

fn get_subreports(report: Vec<N>) -> Input {
    let mut subreports: Input = Vec::new();

    for index in 0..report.len() {
        let subreport = report
            .iter()
            .cloned()
            .take(index)
            .chain(report.iter().skip(index + 1).cloned())
            .collect();

        subreports.push(subreport);
    }

    return subreports;
}

fn solution_2(path: String) -> Result<usize, Box<dyn Error>> {
    let input = read_input(path)?;

    Ok(input
        .into_iter()
        .filter(|report| -> bool {
            let subreports = get_subreports(report.to_owned());

            subreports
                .iter()
                .any(|report| report_is_safe(report.to_owned()))
        })
        .count())
}

fn second() -> Result<(), Box<dyn Error>> {
    let wanted = solution_2("./inputs/02/input.txt".to_owned())?;
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
    use super::{solution_1, solution_2};

    #[test]
    fn solves_example_1_as_expected() {
        let actual = solution_1("./inputs/02/example.txt".to_owned()).unwrap();
        let expected = 2;

        assert_eq!(actual, expected);
    }

    #[test]
    fn solves_example_2_as_expected() {
        let actual = solution_2("./inputs/02/example.txt".to_owned()).unwrap();
        let expected = 4;

        assert_eq!(actual, expected);
    }
}
