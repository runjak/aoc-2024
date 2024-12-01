use std::{error::Error, fs};

type Input = Vec<(i32, i32)>;

fn read_input(path: String) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let lines = input.lines();

    let input = lines
        .filter_map(|line| -> Option<(i32, i32)> {
            let parts = line.split("   ").collect::<Vec<_>>();

            match parts.as_slice() {
                &[a, b] => {
                    let a = a.parse::<i32>().ok()?;
                    let b = b.parse::<i32>().ok()?;

                    return Some((a, b));
                }
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    return Ok(input);
}

fn first() -> Result<(), Box<dyn Error>> {
    println!("To be done.");

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
    use super::read_input;

    #[test]
    fn can_load_example() {
        let actual = read_input("./inputs/01/example.txt".to_owned()).unwrap();
        let expected = [(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)].to_vec();

        assert_eq!(actual, expected);
    }
}
