use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};
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

    let mut current_numbers: HashMap<String, bool> = HashMap::new();
    let mut current = String::new();
    let mut have_symbol = false;

    chars.iter().enumerate().for_each(|(index, char)| {
        let current_col = index % row_width;
        let current_row = index / col_height;
        // println!(
        //     "Char {} {}  [Row: {}, col{}] (MAX [Row: {}, Col: {}])",
        //     index, char, current_row, current_col, row_width, col_height
        // );

        if char.is_digit(10) {
            // we know that we are on the first digit och a number
            // calculate current number we a matching against
            let mut j = index;

            if current.len() == 0 {
                loop {
                    current = format!("{}{}", current, chars[j]);
                    if !chars[j + 1].is_digit(10)
                        || chars[j + 1] == '.'
                        || j % row_width == row_width - 1
                    // hack to handle number ending the row and another starting on next row
                    {
                        break;
                    }
                    j += 1;
                }
            }

            // check left
            have_symbol = if current_col > 0 && is_symbol(&chars, index - 1) {
                // check left
                true
            } else if current_col < row_width - 1 && is_symbol(&chars, index + 1) {
                // check right
                true
            } else {
                have_symbol
            };

            // check up
            if current_row > 0 && is_symbol(&chars, index - col_height) {
                have_symbol = true;
            }
            // check up left
            if current_row > 0
                && current_col > 0
                && chars[index - col_height - 1].is_ascii_punctuation()
                && chars[index - col_height - 1] != '.'
            {
                have_symbol = true;
            }
            // check up right
            if current_row > 0
                && current_col < row_width - 1
                && chars[index - col_height + 1].is_ascii_punctuation()
                && chars[index - col_height + 1] != '.'
            {
                have_symbol = true;
            }
            // check down

            if current_row < col_height - 1
                && chars[index + col_height].is_ascii_punctuation()
                && chars[index + col_height] != '.'
            {
                have_symbol = true;
            }
            // check down left
            if current_row < col_height - 1
                && current_col > 0
                && chars[index + col_height - 1].is_ascii_punctuation()
                && chars[index + col_height - 1] != '.'
            {
                have_symbol = true;
            }
            // check down right
            if current_row < col_height - 1
                && current_col < row_width
                && chars[index + col_height + 1].is_ascii_punctuation()
                && chars[index + col_height + 1] != '.'
            {
                have_symbol = true;
            }

            if chars[index + 1].is_ascii_punctuation() || current_col == row_width - 1 {
                if have_symbol {
                    let val = current.parse::<u32>().ok().unwrap();

                    res += val;
                }
                println!("Done with {} - {}", current, have_symbol);
                current = "".to_string();
                have_symbol = false;
            }
        }
    });
    println!("Row width: {}", row_width);
    println!("Cols Height: {}", col_height);

    println!("{}", current);
    println!("Row {}", row_width);
    println!("Col {}", col_height);

    let result = current_numbers
        .into_iter()
        .filter_map(|(k, v)| {
            if v {
                let mut splitted = k.split("_");
                return Some::<String>(splitted.nth(1).unwrap().to_string());
            }
            None::<String>
        })
        .map(|k| k.parse::<u32>().ok().unwrap())
        .sum::<u32>();

    // part one 530849
    println!("Result {}", result);
    println!("Score {}", res);
    Ok(())
}

fn is_symbol(characters: &Vec<char>, index: usize) -> bool {
    characters[index].is_ascii_punctuation() && characters[index] != '.'
}
