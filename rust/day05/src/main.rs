use std::{fs::File, io::Read, time::Instant};
fn main() {
    let start = Instant::now();

    // result part one - 240320250 - Time elapsed in total is: 549.625µs
    // result part two - 28580589 Time elapsed in total is: 117.383448791s
    part_one();

    let duration = start.elapsed();

    println!("Time elapsed in total is: {:?}", duration);
}
#[derive(Debug)]
struct MapValue {
    source: i64,
    dest: i64,
    length: i64,
}

fn part_one() {
    let mut file = File::open("./src/input.txt").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("failed reading to string");

    let first_line = content.lines().take(1).collect::<String>();
    let seeds: Vec<i64> = get_seeds_v2(first_line);

    let start_index_section_map: Vec<usize> = get_start_index_for_map(&content);
    let almanac = get_almanac(&content, &start_index_section_map);

    println!("Number of seeds  {}", seeds.len());

    let mut res = seeds
        .iter()
        .map(|seed| process_seed(*seed, &almanac))
        .collect::<Vec<i64>>();

    res.sort();
    println!("Results {:?}", res.first());
}

fn process_seed(mut input: i64, almanac: &Vec<Vec<MapValue>>) -> i64 {
    let res = almanac
        .iter()
        .map(|map| {
            input = get_output(input, map);
            input
        })
        .last()
        .unwrap();
    res
}

fn get_output(input: i64, map: &Vec<MapValue>) -> i64 {
    let res = map
        .iter()
        .find(|val| input >= val.source && input < val.source + val.length)
        .map(|val| {
            let diff = val.dest - val.source;
            return input + diff;
        });
    match res {
        Some(val) => val,
        None => input,
    }
}

fn get_almanac(content: &String, start_index_section_map: &Vec<usize>) -> Vec<Vec<MapValue>> {
    (0..start_index_section_map.len())
        .into_iter()
        .enumerate()
        .map(|(i, _section_index)| {
            let last_index = i + 1;
            let end_index;
            if last_index >= start_index_section_map.len() {
                end_index = content.len()
            } else {
                end_index = start_index_section_map[i + 1] - 3;
            }

            let start_index = start_index_section_map[i];

            let lines = content
                .lines()
                .enumerate()
                .filter_map(|(index, line)| {
                    if index >= start_index && index <= end_index {
                        return Some(line);
                    }
                    None
                })
                .map(|numbers| {
                    let n = numbers
                        .split(" ")
                        .into_iter()
                        .filter_map(|n| n.parse::<i64>().ok())
                        .collect::<Vec<i64>>();

                    return MapValue {
                        source: n[1],
                        dest: n[0],
                        length: n[2],
                    };
                })
                .collect::<Vec<_>>();

            lines
        })
        .collect::<Vec<Vec<MapValue>>>()
}

fn get_start_index_for_map(content: &String) -> Vec<usize> {
    let mut start_index_section_map: Vec<usize> = Vec::new();
    content.lines().enumerate().for_each(|(index, line)| {
        line.matches("map")
            .for_each(|_m| start_index_section_map.push(index + 1))
    });
    return start_index_section_map;
}

fn get_seeds_v2(line: String) -> Vec<i64> {
    let seed_line = line.split(": ").nth(1).expect("expected to get seeds");
    println!("Seed line {}", seed_line);

    let slice_seeds = seed_line
        .split(" ")
        .into_iter()
        .map(|c| c.parse::<i64>().ok().unwrap())
        .collect::<Vec<i64>>();

    let seeds = slice_seeds
        .as_slice()
        .chunks(2)
        .into_iter()
        .map(|part| {
            let start = part.get(0).unwrap();
            let range = part.get(1).unwrap();

            let mut seeds: Vec<i64> = Vec::new();
            (0..*range).into_iter().for_each(|i| seeds.push(start + i));

            println!("Done with seed");
            seeds
        })
        .collect::<Vec<Vec<i64>>>();

    seeds.into_iter().flatten().collect()
}

fn get_seeds(line: String) -> Vec<i64> {
    let seed_line = line.split(": ").nth(1).expect("expected to get seeds");
    println!("Seed line {}", seed_line);
    seed_line
        .split(" ")
        .into_iter()
        .map(|c| c.parse::<i64>().ok().unwrap())
        .collect::<Vec<i64>>()
}
