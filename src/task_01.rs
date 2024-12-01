use std::{error::Error, fs};

type N = i32;
type Input = Vec<(N, N)>;

fn read_input(path: String) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let lines = input.lines();

    let input = lines
        .filter_map(|line| -> Option<(N, N)> {
            let parts = line.split("   ").collect::<Vec<_>>();

            match parts.as_slice() {
                &[a, b] => {
                    let a = a.parse::<N>().ok()?;
                    let b = b.parse::<N>().ok()?;

                    return Some((a, b));
                }
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    return Ok(input);
}

fn sorted_columns(input: Input) -> (Vec<N>, Vec<N>) {
    let mut xs = input.iter().map(|(a, _)| a.to_owned()).collect::<Vec<_>>();
    let mut ys = input.iter().map(|(_, b)| b.to_owned()).collect::<Vec<_>>();

    xs.sort();
    ys.sort();

    (xs, ys)
}

fn sum_of_deltas(xs: Vec<N>, ys: Vec<N>) -> N {
    xs.into_iter().zip(ys).map(|(x, y)| (x - y).abs()).sum()
}

fn solution_1(path: String) -> Result<N, Box<dyn Error>> {
    let input = read_input(path)?;
    let (xs, ys) = sorted_columns(input);
    Ok(sum_of_deltas(xs, ys))
}

fn first() -> Result<(), Box<dyn Error>> {
    let wanted  = solution_1("./inputs/01/1.txt".to_owned())?;
    println!("{}",wanted);
    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be done.");

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
    use super::{read_input, solution_1};

    #[test]
    fn can_load_example() {
        let actual = read_input("./inputs/01/example.txt".to_owned()).unwrap();
        let expected = [(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)].to_vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn solves_example_as_expected() {
        let actual = solution_1("./inputs/01/example.txt".to_owned()).unwrap();
        let expected = 11;

        assert_eq!(actual, expected)
    }
}
