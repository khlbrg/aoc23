use regex::Regex;
use std::{
    fs::File,
    io::{Read, Result},
};

const ALLOWED_RED: u32 = 12;
const ALLOWED_GREEN: u32 = 13;
const ALLOWED_BLUE: u32 = 14;

fn main() -> Result<()> {
    // 2237
    match part_one() {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    };

    // 66681
    match part_two() {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }

    Ok(())
}

fn part_one() -> Result<()> {
    let mut file = File::open("./src/example1.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let green_re = Regex::new(r"(\d{1,}) (green)").unwrap();
    let red_re = Regex::new(r"(\d{1,}) (red)").unwrap();
    let blue_re = Regex::new(r"(\d{1,}) (blue)").unwrap();

    let mut points = 0;

    content.lines().enumerate().for_each(|(k, val)| {
        let red_valid = is_cubes_valid(&red_re, val, ALLOWED_RED);
        let green_valid = is_cubes_valid(&green_re, val, ALLOWED_GREEN);
        let blue_valid = is_cubes_valid(&blue_re, val, ALLOWED_BLUE);

        if red_valid && green_valid && blue_valid {
            points += k + 1
        }
    });

    println!("Part One: {}", points);
    Ok(())
}

fn is_cubes_valid(re: &Regex, line: &str, max_allowed: u32) -> bool {
    re.captures_iter(&line)
        .filter_map(|m| m.get(1).unwrap().as_str().parse::<u32>().ok())
        .max()
        .into_iter()
        .map(|max_val| max_val.cmp(&max_allowed).is_le())
        .next()
        .unwrap()
}

fn get_max(re: &Regex, line: &str) -> u32 {
    re.captures_iter(line)
        .filter_map(|m| m.get(1).unwrap().as_str().parse::<u32>().ok())
        .max_by(|x, y| x.cmp(y))
        .unwrap()
}

fn part_two() -> Result<()> {
    let mut file = File::open("./src/example2.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let green_re = Regex::new(r"(\d{1,}) (green)").unwrap();
    let red_re = Regex::new(r"(\d{1,}) (red)").unwrap();
    let blue_re = Regex::new(r"(\d{1,}) (blue)").unwrap();

    let mut points = 0;

    content.lines().enumerate().for_each(|(_k, val)| {
        let max_green = get_max(&green_re, &val);
        let max_blue: u32 = get_max(&blue_re, &val);
        let max_red: u32 = get_max(&red_re, &val);

        points += max_green * max_blue * max_red;
    });

    println!("Part two: {}", points);
    Ok(())
}
