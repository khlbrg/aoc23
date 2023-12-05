use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    println!("Hello, world!");
    part_two();

    //part_one();

    Ok(())
}
#[derive(Debug, Clone)]
struct Card {
    winning: Vec<u32>,
    your: Vec<u32>,
}

pub fn part_two() {
    let mut file = File::open("./src/example.txt").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("failed reading to string");

    let mut played_games: Vec<u32> = Vec::new();
    // generate games
    content
        .lines()
        .map(|line| {
            let line_string = line.to_string();
            let parts = line_string.split(": ").nth(1).unwrap();
            let number_parts: Vec<_> = parts.trim().split(" | ").collect();

            let winning_number = number_parts[0]
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            let your_number = number_parts[1]
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            played_games.push(0);

            Card {
                winning: winning_number,
                your: your_number,
            }
        })
        .collect::<Vec<Card>>()
        .iter_mut()
        .enumerate()
        .for_each(|(i, card)| {
            played_games[i] += 1;

            let matches = card
                .your
                .iter()
                .filter_map(|num| {
                    if card.winning.contains(&num) {
                        Some(num)
                    } else {
                        None
                    }
                })
                .count();

            //println!("Playing for idx: {} - current: {:?}", i, played_games[i]);
            for x in 0..matches {
                played_games[i + 1 + x] += played_games[i];
            }
        });
    // 6227972
    println!("Played games {:?} ", played_games.iter().sum::<u32>());
}

pub fn part_one() -> usize {
    let mut file = File::open("./src/example.txt").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).expect("failed to read");

    let result = content
        .lines()
        .map(|line| {
            let line_string = line.to_string();
            let parts: Vec<_> = line_string.split(": ").collect();
            let number_parts: Vec<_> = parts[1].trim().split(" | ").collect();

            let winning_number = number_parts[0]
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            let your_number = number_parts[1]
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            return (winning_number, your_number);
        })
        .map(|(winning, your)| {
            let matches = your
                .iter()
                .filter_map(|num| {
                    if winning.contains(&num) {
                        Some(num)
                    } else {
                        None
                    }
                })
                .count();

            let mut res = 1;
            (0..matches).for_each(|_| res *= 2);
            res / 2
        })
        .sum();

    println!("Result {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 13);
    }
}
