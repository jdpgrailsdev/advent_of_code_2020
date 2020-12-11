use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() {
    let data = load_data();
	
	let seating_chart = compute(&mut data);
}

fn compute(data: &mut Vec<Vec<char>>) {
	let mut current_row = 0;
	let mut current_col = 0;
	let mut changes = 0;

	for row in data.iter() {
		for seat in row {
			let open_seats = check_open_seats(data.to_vec(), current_row, current_col);		
			println!("There are {} open seats around seat {} in row {}.", open_seats, current_col, current_row);
			if *seat == 'L' {
				if open_seats == 8 {
					data[current_row as usize][current_col as usize] = '#';
					changes += 1;
				} else {
					
				}
			} else if *seat == '#' {
				if open_seats <= 4 {
					data[current_row as usize][current_col as usize] = 'L';
					changes += 1;
				} 
			}
			
			current_col += 1;
		}
		
		current_row += 1;
		current_col = 0;
	}
	
	if changes > 0 {
		compute(data);
	} else {
		let mut occupied = 0;
		for row in data {
			for seat in row {
				println!("Looing at seat {}", seat);
				if *seat == '#' {
					occupied += 1;
				}
			}
		}
		println!("There are {} occupied seats.", occupied);
	}	
}

fn check_open_seats(data: Vec<Vec<char>>, current_row: i32, current_col: i32) -> i32 {
	let mut open_seats = 0;
	
	for r in [(current_row - 1),(current_row + 1)].iter() {		
		if r < &0 || r >= &(data.len() as i32) {
		    open_seats += 1;
		} else {						 
			for c in [(current_col - 1),(current_col + 1)].iter() {
				let row = data.get(*r as usize).unwrap();
				if c < &0 || c >= &(row.len() as i32) {		
					open_seats += 1;
				} else {		
					let seat = row.get(*c as usize).unwrap();
					if *seat != '#' {
						open_seats += 1;
					}
				}				
			}
		}
	}
		
	return open_seats;
}

fn load_data() -> Vec<Vec<char>> {
	let input = File::open("./data/day11.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<Vec<char>> = reader.lines()
		.map(|l| { l.unwrap().chars().collect() })
		.collect();

	return data;
}