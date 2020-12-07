use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;
use math::round;

pub fn exercise() {
    let mut floor_plan: Vec<Vec<char>> = vec![vec!['_'; 8]; 128]
    let mut seat_ids: Vec<i32> = Vec::new();
    let data = load_data();

    for ticket in data {
        let mut row_position = (0,127);
        row_position = find(ticket[..7].chars(), row_position.0, row_position.1);
        let row = row_position.0;

        let mut col_position = (0,7);
        col_position = find(ticket[7..].chars(), col_position.0, col_position.1);
        let col = col_position.0;

        seat_ids.push((row * 8) + col);

        let _ignore = std::mem::replace(&mut floor_plan[row as usize][col as usize], 'X');
    }

    println!("Highest seat ID value is {}.", seat_ids.iter().max().unwrap());

    for (i, row) in floor_plan.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            if *col != 'X' {
                let id = (i*8)+y;
                if id > 0 && seat_ids.iter().any(|i| *i == ((id - 1) as i32 )) && seat_ids.iter().any(|i| *i == ((id + 1) as i32)) {
                    println!("My seat ID is {}.", id);
                }
            }
        }
    }
}

fn find(mut ticket: Chars, lower: i32, upper: i32) -> (i32, i32) {
    if ticket.as_str() == "" {
        return (lower, upper);
    } else {
        let indicator = ticket.next().unwrap();
        if indicator == 'F'  || indicator == 'L' {
            return find(ticket, lower, floor_division((upper+lower) as f64, 2 as f64));
        } else if indicator == 'B' || indicator == 'R' {
            return find(ticket, ceil_division((upper+lower) as f64, 2 as f64), upper);
        } else {
            println!("Unsupported indicator '{}'.", indicator);
            return (lower, upper);
        }
    }
}

fn ceil_division(divisor:f64, dividend:f64) -> i32 {
    return round::ceil(divisor/dividend, 0) as i32;
}

fn floor_division(divisor:f64, dividend:f64) -> i32 {
    return round::floor(divisor/dividend, 0) as i32;
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day5.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}