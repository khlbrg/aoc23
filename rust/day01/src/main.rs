use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    match part_one() {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }

    match part_two() {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }

    Ok(())
}

fn part_one() -> Result<()> {
    let mut file = File::open("./src/example.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let result = content
        .lines()
        .map(|line| {
            // Strip all non numeric values from the string
            let mut line_with_digits = line.chars().filter_map(|character| character.to_digit(10));

            // get the first value, unwrap because we always know it exist
            let first_val = line_with_digits.next().unwrap();

            // last value may not exist, becau
            let last_val = match line_with_digits.last() {
                Some(num) => num,
                None => first_val,
            };

            format!("{}{}", first_val, last_val).parse::<u32>().unwrap()
        })
        .sum::<u32>();

    println!("Part one result: {}", result);

    Ok(())
}

fn part_two() -> Result<()> {
    let mut file = File::open("./src/example2.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let numbers = HashMap::from([("one", '1'), ("two", '2'), ("three", '3'), ("four", '4')]);

    let result = content
        .lines()
        .map(|line| {
            let mut new_line = (0..line.len()).filter_map(|idx| {
                let current_line = &line[idx..];

                let result: char = if current_line.starts_with("one") {
                    '1'
                } else if current_line.starts_with("two") {
                    '2'
                } else if current_line.starts_with("three") {
                    '3'
                } else if current_line.starts_with("four") {
                    '4'
                } else if current_line.starts_with("five") {
                    '5'
                } else if current_line.starts_with("six") {
                    '6'
                } else if current_line.starts_with("seven") {
                    '7'
                } else if current_line.starts_with("eight") {
                    '8'
                } else if current_line.starts_with("nine") {
                    '9'
                } else {
                    current_line.chars().next().unwrap()
                };

                result.to_digit(10)
            });

            // Strip all non numeric values from the string
            //let mut line_with_digits = line.chars().filter_map(|character| character.to_digit(10));

            // get the first value, unwrap because we always know it exist
            let first_val = new_line.next().unwrap();

            // last value may not exist, becau
            let last_val = match new_line.last() {
                Some(num) => num,
                None => first_val,
            };

            format!("{}{}", first_val, last_val).parse::<u32>().unwrap()
        })
        .sum::<u32>();

    println!("Part two result: {}", result);

    Ok(())
}

/*

let mut file = File::open("./src/example.txt")?;
let mut file_content = String::new();

file.read_to_string(&mut file_content)?;
let result: u32 = file_content
    .lines()
    .map(|line| {
        let mut it = line.chars().filter_map(|c| c.to_digit(10));

        let first = it.next().expect("should be a number");
        let last = it.last();

        match last {
            Some(num) => format!("{}{}", first, num),
            None => format!("{}{}", first, first),
        }
        .parse::<u32>()
        .unwrap()
    })
    .sum::<u32>();

println!("{:?}", result);

Ok(()) */
