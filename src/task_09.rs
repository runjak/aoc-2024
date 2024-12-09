use std::{error::Error, fs};

const INPUT_PATH: &str = "./inputs/09/input.txt";

type N = i64;

#[derive(Clone, Copy, PartialEq, Debug)]
struct File {
    id: N,
    length: N,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Segment {
    Space(N),
    File(File),
}

type Input = Vec<Segment>;

fn read_input(path: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let mut is_space = false;

    Ok(input
        .chars()
        .enumerate()
        .filter_map(|(index, char)| -> Option<Segment> {
            let length = char.to_digit(10)? as N;

            let segment = match is_space {
                true => Segment::Space(length),
                false => Segment::File(File {
                    id: (index as N) / 2,
                    length,
                }),
            };

            is_space = !is_space;

            Some(segment)
        })
        .collect())
}

fn compact(mut input: Input) -> Vec<File> {
    let mut compacted: Vec<File> = Vec::new();
    let mut first_file_id: N = -1; // index / 2, we can simplify?

    for index in 0..input.len() {
        match input.get(index) {
            Some(Segment::File(file)) => {
                compacted.push(*file);
                first_file_id = file.id;
            }
            Some(Segment::Space(space_to_fill)) => {
                let mut space_to_fill = *space_to_fill;

                while space_to_fill > 0 {
                    match input.pop() {
                        Some(Segment::File(mut file)) => {
                            if file.id <= first_file_id {
                                return compacted;
                            }

                            let to_take = space_to_fill.min(file.length);
                            compacted.push(File {
                                id: file.id,
                                length: to_take,
                            });

                            file.length -= to_take;
                            if file.length > 0 {
                                input.push(Segment::File(file));
                            }

                            space_to_fill -= to_take;
                        }
                        Some(Segment::Space(_)) => {}
                        None => {
                            return compacted;
                        }
                    }
                }
            }
            None => {
                return compacted;
            }
        }
    }

    compacted
}

fn checksum_files(files: &Vec<File>) -> N {
    let mut offset: N = 0;

    files
        .iter()
        .map(|file| -> N {
            let mut sum: N = 0;

            /*
             * Optimisable. Consider:
             * nx + (n+1)x + .. + (n+(l-1))x
             * = l(x+n) + (l*(l-1))/2
             */
            for _ in 0..file.length {
                sum += offset * file.id;
                offset += 1;
            }

            sum
        })
        .sum()
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = checksum_files(&compact(input));
    println!("{}", wanted);
    Ok(())
}

fn compact_whole(mut input: Input) -> Input {
    let mut start_seek_free = 0;

    for file_index in (0..input.len()).rev() {
        if file_index < start_seek_free {
            break;
        }

        let Some(Segment::File(file)) = input.get(file_index) else {
            continue;
        };
        let file = *file;

        let mut found_first_free = false;
        for free_index in start_seek_free..file_index {
            let Some(Segment::Space(space)) = input.get(free_index) else {
                continue;
            };

            if !found_first_free {
                found_first_free = true;
                start_seek_free = free_index;
            }

            let left_over_space = *space - file.length;
            if left_over_space < 0 {
                continue;
            }

            input[file_index] = Segment::Space(file.length);
            input[free_index] = Segment::File(file);
            if left_over_space > 0 {
                input.insert(free_index + 1, Segment::Space(left_over_space));
            }
            break;
        }
    }

    input
}

fn checksum_segments(input: Input) -> N {
    let mut offset: N = 0;

    input
        .iter()
        .map(|segment| -> N {
            let mut sum: N = 0;
            match segment {
                Segment::Space(space) => {
                    offset += space;
                }
                Segment::File(file) => {
                    /*
                     * Optimisable. Consider:
                     * nx + (n+1)x + .. + (n+(l-1))x
                     * = l(x+n) + (l*(l-1))/2
                     */
                    for _ in 0..file.length {
                        sum += offset * file.id;
                        offset += 1;
                    }
                }
            }

            sum
        })
        .sum()
}

fn second() -> Result<(), Box<dyn Error>> {
    let input = read_input(INPUT_PATH)?;
    let wanted = checksum_segments(compact_whole(input));
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
    use crate::task_09::checksum_segments;

    use super::{checksum_files, compact, compact_whole, read_input};

    const EXAMPLE_PATH: &str = "./inputs/09/example.txt";

    #[test]
    fn should_compact_example_1() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let compacted = compact(input);

        let actual = compacted
            .into_iter()
            .flat_map(|file| -> Vec<char> {
                file.id
                    .to_string()
                    .repeat(file.length as usize)
                    .chars()
                    .collect()
            })
            .collect::<String>();

        let expected = "0099811188827773336446555566".to_owned();
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_solve_example_1() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let compacted = compact(input);
        let actual = checksum_files(&compacted);

        let expected = 1928;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_compact_example_2() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let compacted = compact_whole(input);

        let actual = compacted
            .into_iter()
            .flat_map(|segment| -> Vec<char> {
                match segment {
                    super::Segment::Space(space) => ".".repeat(space as usize).chars().collect(),
                    super::Segment::File(file) => file
                        .id
                        .to_string()
                        .repeat(file.length as usize)
                        .chars()
                        .collect(),
                }
            })
            .collect::<String>();

        let expected = "00992111777.44.333....5555.6666.....8888..".to_owned();
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_solve_example_2() {
        let input = read_input(EXAMPLE_PATH).unwrap();
        let compacted = compact_whole(input);
        let actual = checksum_segments(compacted);

        let expected = 2858;
        assert_eq!(actual, expected);
    }
}
