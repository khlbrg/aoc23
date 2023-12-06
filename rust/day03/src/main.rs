use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Result},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    idx: usize,
    number: String,
}
fn main() -> Result<()> {
    let start = Instant::now();

    let mut file = File::open("./src/example1.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let mut grid_width = 0;
    let mut grid_height = 0;
    let mut chars = Vec::new();
    let mut res = 0;

    // build a vec for all characters in the grid
    // calculate width and height to later calculate current
    // col and row when iterating
    content.lines().for_each(|line| {
        grid_width = line.len();
        grid_height += 1;

        line.chars().for_each(|c| chars.push(c))
    });

    let mut current = String::new();
    let mut have_adjecent_symbol = false;

    // vec to store all numbers and it's index
    let mut numbers_in_grid: Vec<Number> = Vec::new();

    chars.iter().enumerate().for_each(|(index, char)| {
        let current_col = index % grid_width;
        let current_row = index / grid_height;

        if char.is_digit(10) {
            let mut j = index;

            // This parts calculate the current number
            // We know we are at first digit of a number, iterate right until
            // we find a punctuation character or when we go outside right side of the grid
            if current.len() == 0 {
                loop {
                    current = format!("{}{}", current, chars[j]);
                    if chars[j + 1].is_ascii_punctuation()
                        || chars[j + 1] == '.'
                        || j % grid_width == grid_width - 1
                    // hack to handle number ending the row and another starting on next row
                    {
                        break;
                    }
                    j += 1;
                }
            }

            // check left
            have_adjecent_symbol = if current_col > 0 && is_symbol(&chars, index - 1) {
                // check left
                true
            } else if current_col < grid_width - 1 && is_symbol(&chars, index + 1) {
                // check right
                true
            } else if current_row > 0 && is_symbol(&chars, index - grid_height) {
                // check up
                true
            } else if current_row > 0
                && current_col > 0
                && is_symbol(&chars, index - grid_height - 1)
            {
                // check up left
                true
            } else if current_row > 0
                && current_col < grid_width - 1
                && is_symbol(&chars, index - grid_height + 1)
            {
                // check up right
                true
            } else if current_row < grid_height - 1 && is_symbol(&chars, index + grid_height) {
                // check down
                true
            } else if current_row < grid_height - 1
                && current_col > 0
                && is_symbol(&chars, index + grid_height - 1)
            {
                // check down left
                true
            } else if current_row < grid_height - 1
                && current_col < grid_width
                && is_symbol(&chars, index + grid_height + 1)
            {
                // check down right
                true
            } else {
                have_adjecent_symbol
            };

            let current_number = Number {
                number: current.clone(),
                idx: index,
            };

            // push the number into grid, to be used in part 2
            numbers_in_grid.push(current_number);

            // Now we know we have processed the last digit in the number
            // Save all adjcent symbols we encountered during the iteration
            // and parse the current number and add to result
            // reset current number and have_adjecent_symbol
            if chars[index + 1].is_ascii_punctuation() || current_col == grid_width - 1 {
                if have_adjecent_symbol {
                    let val = current.parse::<u32>().ok().unwrap();
                    res += val;
                }

                current = "".to_string();
                have_adjecent_symbol = false;
            }
        }
    });

    let duration = start.elapsed();
    println!("Time elapsed in part oneis: {:?}", duration);

    // part two
    // iterate all characters until we find a *
    // then check every direction and match the index against the
    // numbers_in_grid
    // we will get duplicates, but will sort all duplicates out. In my case case there was
    // no same number adjecent to the same symbol, in that case you need to compare start_x and start_y
    // for each number as well
    let mut part_two_result = 0;
    chars.iter().enumerate().for_each(|(index, char)| {
        let current_col = index % grid_width;
        let current_row = index / grid_height;

        if *char == '*' {
            let left_idx = index - 1;
            let right_idx = index + 1;
            let up_idx = index - grid_height;
            let up_left_idx = index - grid_height - 1;
            let up_right_idx = index - grid_height + 1;
            let down_idx = index + grid_height;
            let down_left_idx = index + grid_height - 1;
            let down_right_idx = index + grid_height + 1;

            let mut adjecent_numbers: Vec<&Number> = Vec::new();

            // check left
            if current_col > 0 && is_d(&chars, left_idx) {
                match get_number_at_index(&numbers_in_grid, left_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }
            // check right
            if current_col < grid_width - 1 && is_d(&chars, right_idx) {
                match get_number_at_index(&numbers_in_grid, right_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

            // check up
            if current_row > 0 && is_d(&chars, up_idx) {
                match get_number_at_index(&numbers_in_grid, up_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

            // check up left
            if current_row > 0 && is_d(&chars, up_left_idx) {
                match get_number_at_index(&numbers_in_grid, up_left_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

            // check up right
            if current_row > 0 && current_col < grid_width - 1 && is_d(&chars, up_right_idx) {
                match get_number_at_index(&numbers_in_grid, up_right_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }
            // check down
            if current_row < grid_height - 1 && is_d(&chars, down_idx) {
                match get_number_at_index(&numbers_in_grid, down_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

            // check down left
            if current_row < grid_height - 1 && current_col > 0 && is_d(&chars, down_left_idx) {
                match get_number_at_index(&numbers_in_grid, down_left_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

            // check down right
            if current_row < grid_height - 1
                && current_col < grid_width
                && is_d(&chars, down_right_idx)
            {
                match get_number_at_index(&numbers_in_grid, down_right_idx) {
                    Some(x) => adjecent_numbers.push(x),
                    None => (),
                };
            }

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
                part_two_result += first * second;
            }
        }
    });

    // part one 530849
    println!("Score {}", res);

    // Correct         84900879
    println!("Score part two: {}", part_two_result);

    let duration = start.elapsed();

    println!("Time elapsed in total is: {:?}", duration);
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
