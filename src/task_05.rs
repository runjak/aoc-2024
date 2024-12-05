use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

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

    let page_orderings = page_orderings
        .lines()
        .filter_map(|line| -> Option<PageOrdering> {
            let [before, after] = line
                .split("|")
                .filter_map(|item| -> Option<Page> { item.parse::<Page>().ok() })
                .collect::<Vec<_>>()[..]
            else {
                return None;
            };

            Some(PageOrdering { before, after })
        })
        .collect::<Vec<_>>();

    let updates = updates
        .lines()
        .map(|line| -> Update {
            line.split(",")
                .filter_map(|page| -> Option<Page> { page.parse().ok() })
                .collect()
        })
        .collect::<Vec<_>>();

    Ok((page_orderings, updates))
}

type BeforeMap = HashMap<Page, HashSet<Page>>;

fn before_map(page_orderings: Vec<PageOrdering>) -> BeforeMap {
    // We find values that must happen before the others.
    let mut after_to_befores: BeforeMap = HashMap::new();

    for PageOrdering { before, after } in page_orderings.iter() {
        match after_to_befores.get_mut(after) {
            Some(befores) => {
                befores.insert(*before);
            }
            None => {
                after_to_befores.insert(*after, HashSet::from([*before]));
            }
        }
    }

    return after_to_befores;
}

fn filter_ordered_updates(after_to_befores: BeforeMap, updates: Vec<Update>) -> Vec<Update> {
    updates
        .into_iter()
        .filter(|update| -> bool {
            for (index, page) in update.iter().enumerate() {
                let Some(befores) = after_to_befores.get(page) else {
                    continue;
                };

                let afters = &update[index + 1..];
                if afters.iter().any(|after| befores.contains(after)) {
                    return false;
                }
            }

            return true;
        })
        .collect()
}

fn get_middle(update: Update) -> Page {
    let index = update.len() / 2;
    *update.get(index).unwrap_or(&0)
}

fn solution_1(input: Input) -> Page {
    let ordered_updates = filter_ordered_updates(before_map(input.0), input.1);

    ordered_updates
        .into_iter()
        .map(|update| get_middle(update))
        .sum::<Page>()
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH).unwrap();
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
    use super::{before_map, filter_ordered_updates, read_input, solution_1, Update};

    const EXAMPLE_PATH: &str = "./inputs/05/example.txt";

    #[test]
    fn should_find_ordered_updates() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = filter_ordered_updates(before_map(input.0), input.1);

        let expected: Vec<Update> = Vec::from([
            Vec::from([75, 47, 61, 53, 29]),
            Vec::from([97, 61, 53, 29, 13]),
            Vec::from([75, 29, 13]),
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_first_example() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let actual = solution_1(input);
        let expected = 143;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_second_example() {
        assert_eq!(1, 0);
    }
}
