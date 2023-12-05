use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    idx: usize,
    start_x: usize,
    end_x: usize,
    number: String,
}
fn main() -> Result<()> {
    println!("Hello, world!");
    let mut file = File::open("./src/example1.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let mut row_width = 0;
    let mut col_height = 0;
    let mut chars = Vec::new();
    let mut res = 0;

    content.lines().for_each(|line| {
        row_width = line.len();

        col_height += 1;

        line.chars().for_each(|c| chars.push(c))
    });

    let mut current = String::new();
    let mut have_symbol = false;
    let mut start_x: usize = 0;
    let mut end_x: usize = 0;
    let mut numbers_in_grid: Vec<Number> = Vec::new();

    chars.iter().enumerate().for_each(|(index, char)| {
        let current_col = index % row_width;
        let current_row = index / col_height;

        if char.is_digit(10) {
            let mut j = index;

            if current.len() == 0 {
                start_x = index;
                loop {
                    current = format!("{}{}", current, chars[j]);
                    if !chars[j + 1].is_digit(10)
                        || chars[j + 1] == '.'
                        || j % row_width == row_width - 1
                    // hack to handle number ending the row and another starting on next row
                    {
                        end_x = j;
                        break;
                    }
                    j += 1;
                }
            }
            if current == "2" {
                println!("Found it {}", current)
            }

            // check left
            have_symbol = if current_col > 0 && is_symbol(&chars, index - 1) {
                // check left
                true
            } else if current_col < row_width - 1 && is_symbol(&chars, index + 1) {
                // check right
                true
            } else if current_row > 0 && is_symbol(&chars, index - col_height) {
                // check up
                true
            } else if current_row > 0
                && current_col > 0
                && is_symbol(&chars, index - col_height - 1)
            {
                // check up left
                true
            } else if current_row > 0
                && current_col < row_width - 1
                && is_symbol(&chars, index - col_height + 1)
            {
                // check up right
                true
            } else if current_row < col_height - 1 && is_symbol(&chars, index + col_height) {
                // check down
                true
            } else if current_row < col_height - 1
                && current_col > 0
                && is_symbol(&chars, index + col_height - 1)
            {
                // check down left
                true
            } else if current_row < col_height - 1
                && current_col < row_width
                && is_symbol(&chars, index + col_height + 1)
            {
                // check down right
                true
            } else {
                have_symbol
            };

            let current_number = Number {
                end_x: end_x,
                start_x: start_x,
                number: current.clone(),
                idx: index,
            };

            numbers_in_grid.push(current_number);

            if chars[index + 1].is_ascii_punctuation() || current_col == row_width - 1 {
                if have_symbol {
                    let val = current.parse::<u32>().ok().unwrap();
                    res += val;
                }

                current = "".to_string();
                have_symbol = false;
                start_x = 0;
                end_x = index;
            }
        }
    });

    // part two
    let mut part_two_result = 0;
    let mut part_two_result_2 = 0;
    let mut stars = 0;
    chars.iter().enumerate().for_each(|(index, char)| {
        let current_col = index % row_width;
        let current_row = index / col_height;

        if *char == '*' {
            let left_idx = index - 1;
            let right_idx = index + 1;
            let up_idx = index - col_height;
            let up_left_idx = index - col_height - 1;
            let up_right_idx = index - col_height + 1;
            let down_idx = index + col_height;
            let down_left_idx = index + col_height - 1;
            let down_right_idx = index + col_height + 1;

            let mut adjecent_numbers: Vec<&Number> = Vec::new();

            // check left
            if current_col > 0 && is_d(&chars, left_idx) {
                match get_number_at_index(&numbers_in_grid, left_idx) {
                    Some(x) => {
                        //println!("Adjecent left {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }
            // check right
            if current_col < row_width - 1 && is_d(&chars, right_idx) {
                match get_number_at_index(&numbers_in_grid, right_idx) {
                    Some(x) => {
                        //println!("Adjecent right {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }

            // check up
            if current_row > 0 && is_d(&chars, up_idx) {
                match get_number_at_index(&numbers_in_grid, up_idx) {
                    Some(x) => {
                        // println!("Adjecent up {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }

            // check up left
            if current_row > 0 && is_d(&chars, up_left_idx) {
                match get_number_at_index(&numbers_in_grid, up_left_idx) {
                    Some(x) => {
                        //println!("Adjecent up left {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }

            // check up right
            if current_row > 0 && current_col < row_width - 1 && is_d(&chars, up_right_idx) {
                match get_number_at_index(&numbers_in_grid, up_right_idx) {
                    Some(x) => {
                        //println!("Adjecent up right {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }
            // check down
            if current_row < col_height - 1 && is_d(&chars, down_idx) {
                match get_number_at_index(&numbers_in_grid, down_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

            // check down left
            if current_row < col_height - 1 && current_col > 0 && is_d(&chars, down_left_idx) {
                match get_number_at_index(&numbers_in_grid, down_left_idx) {
                    Some(x) => {
                        //println!("Adjecent down left {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }

            // check down right
            if current_row < col_height - 1
                && current_col < row_width
                && is_d(&chars, down_right_idx)
            {
                match get_number_at_index(&numbers_in_grid, down_right_idx) {
                    Some(x) => {
                        //println!("Adjecent down right {:?}", x);
                        adjecent_numbers.push(x)
                    }
                    None => (),
                };
            }

            stars += 1;
            let mut result_map: Vec<u32> = Vec::new();
            adjecent_numbers.iter().for_each(|num| {
                let parsed = num.number.parse::<u32>().unwrap();
                if !result_map.contains(&parsed) {
                    result_map.push(parsed)
                }

                adjecent_numbers.iter().for_each(|numtwo| {
                    if num.number == numtwo.number
                        && num.start_x != numtwo.start_x
                        && num.end_x != numtwo.end_x
                    {
                        println!("Found duplicate number {:?} {:?}", num, numtwo);
                    }
                })
            });

            // find unqiue
            let mut numbers = HashSet::new();
            adjecent_numbers
                .iter()
                .filter(|p| numbers.insert(p.number.clone().parse::<u32>().ok().unwrap()))
                .for_each(|x| drop(x));

            let mut nums = numbers.drain();
            if nums.len() == 2 {
                let first = nums.next().unwrap();
                let second = nums.next().unwrap();

                let result = first * second;

                // println!(
                //     "Char:{}[{},{}] {:?} - {} - {}",
                //     char, current_row, current_col, nums, first, second
                // );

                part_two_result += result;
            } else {
                println!(
                    "Skipping:{}[{},{}] {:?} ",
                    char, current_row, current_col, nums,
                );
            }

            if result_map.len() == 2 {
                part_two_result_2 += result_map.get(0).unwrap() * result_map.get(1).unwrap()
            }
        }
    });

    println!("Stars {}", stars);

    println!("Row width: {}", row_width);
    println!("Cols Height: {}", col_height);
    println!("Row {}", row_width);
    println!("Col {}", col_height);

    // part one 530849
    println!("Score {}", res);

    // attemp1: 84899503
    // Correct         84900879
    println!("Score part two: {}", part_two_result);
    println!("Score part two: {}", part_two_result_2);
    Ok(())
}

fn is_symbol(characters: &Vec<char>, index: usize) -> bool {
    characters[index].is_ascii_punctuation() && characters[index] != '.'
}
fn is_d(characters: &Vec<char>, index: usize) -> bool {
    characters[index].is_digit(10)
}
fn get_number_at_index(numbers: &Vec<Number>, index: usize) -> Option<&Number> {
    numbers.into_iter().find(|&x| x.idx == index)
}
// astrid
