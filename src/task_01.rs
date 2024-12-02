use std::{collections::HashMap, error::Error, fs};

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
    let wanted = solution_1("./inputs/01/1.txt".to_owned())?;
    println!("{}", wanted);
    Ok(())
}

fn count_occurrences(xs: Vec<N>) -> HashMap<N, N> {
    let mut occurrences: HashMap<N, N> = HashMap::new();

    for x in xs.iter() {
        if let Some(n) = occurrences.get(x) {
            occurrences.insert(*x, *n + 1);
        } else {
            occurrences.insert(*x, 1);
        }
    }

    return occurrences;
}

fn solution_2(path: String) -> Result<N, Box<dyn Error>> {
    let input = read_input(path)?;
    let (xs, ys) = sorted_columns(input);

    let occurrence_counts = count_occurrences(ys);

    Ok(xs
        .iter()
        .map(|x| -> N {
            let c = occurrence_counts.get(x).unwrap_or(&0);

            return x * c;
        })
        .sum())
}

fn second() -> Result<(), Box<dyn Error>> {
    let wanted = solution_2("./inputs/01/1.txt".to_owned())?;
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
    use std::collections::HashMap;

    use super::{count_occurrences, read_input, solution_1, solution_2, sorted_columns};

    #[test]
    fn can_load_example() {
        let actual = read_input("./inputs/01/example.txt".to_owned()).unwrap();
        let expected = [(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)].to_vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn solves_example_1_as_expected() {
        let actual = solution_1("./inputs/01/example.txt".to_owned()).unwrap();
        let expected = 11;

        assert_eq!(actual, expected)
    }

    #[test]
    fn count_example_occurrences() {
        let input = read_input("./inputs/01/example.txt".to_owned()).unwrap();
        let (_, xs) = sorted_columns(input);
        let actual = count_occurrences(xs);

        let expected = HashMap::from_iter([(3, 3), (9, 1), (5, 1), (4, 1)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn solves_example_2_as_expected() {
        let actual = solution_2("./inputs/01/example.txt".to_owned()).unwrap();
        let expected = 31;

        assert_eq!(actual, expected)
    }
}
