struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    // let races_part_two = vec![Race {
    //     time: 44806572,
    //     distance: 208158110501102,
    // }];
    let races = vec![
        Race {
            time: 44,
            distance: 208,
        },
        Race {
            time: 80,
            distance: 1581,
        },
        Race {
            time: 65,
            distance: 1050,
        },
        Race {
            time: 72,
            distance: 1102,
        },
    ];

    let total_wins = races
        .iter()
        .map(|race| {
            let wins = (0..race.time)
                .into_iter()
                .filter_map(|i| {
                    let time_left_to_travel = race.time - i as u32;
                    let distance = time_left_to_travel * i as u32;
                    if distance > race.distance {
                        Some(true)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            wins.len() as u32
        })
        .product::<u32>();

    println!("Totalt {}", total_wins)
}
