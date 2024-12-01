use std::{error::Error, fs};

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
