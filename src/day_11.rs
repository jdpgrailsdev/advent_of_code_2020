use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() {
    let mut data = load_data();
	
//	compute(&mut data);
}

fn compute(data: &mut Vec<Vec<char>>) {
	let mut current_row = 0;
	let mut current_col = 0;
	let mut changes = 0;
	let mut updated_rows: Vec<Vec<char>> = data.clone();
	let length = updated_rows.len();

	for r in 0..length {
		let mut updated_seats: Vec<char> = vec![];
		let row = updated_rows.get(r).unwrap();
		
		for seat in row {
			let open_seats = check_open_seats(updated_rows.to_vec(), current_row, current_col);		
//			println!("There are {} open seats around seat {} in row {}.", open_seats, current_col, current_row);
			if *seat == 'L' {
				if open_seats == 8 {
					updated_seats.push('#');
					changes += 1;
				} else {
					updated_seats.push(*seat);
				}
			} else if *seat == '#' {
				if open_seats <= 4 {
					updated_seats.push('L');
					changes += 1;
				} else {
					updated_seats.push(*seat);
				}
			} else {
				updated_seats.push(*seat);
			}
			
			current_col += 1;
		}
		
		updated_rows.push(updated_seats);
		
		current_row += 1;
		current_col = 0;
	}
	
	if changes > 0 {
		println!("Changes ({}) were made...making another pass.", changes);
		compute(&mut updated_rows);
	} else {
		let mut occupied = 0;
		for row in updated_rows {
			for seat in row {
				if seat == '#' {
					occupied += 1;
				}
			}
		}
		println!("There are {} occupied seats.", occupied);
	}	
}

fn check_open_seats(data: Vec<Vec<char>>, current_row: i32, current_col: i32) -> i32 {
	let mut open_seats = 0;
	let row_range: Vec<i32> = ((current_row - 1)..(current_row + 2)).collect();
	let col_range: Vec<i32> = ((current_col - 1)..(current_col + 2)).collect();
	
//	println!("{:?} and {:?}", row_range, col_range);
	
	for r in row_range {		
		if r >= 0 && r < (data.len() as i32) {				 
			for c in &col_range {
				if (r, *c) != (current_row, current_col) {
					let row = data.get(r as usize).unwrap();
					if *c >= 0 && *c < (row.len() as i32) {		
						let seat = row.get(*c as usize).unwrap();
//						println!("Checking seat '{}' at position ({},{})...", seat, r, c);
						if *seat != '#' {
							open_seats += 1;
						}
					}
				}				
			}
		}
	}
	
//	println!("Found {} open_seats for row {} and col {}.", open_seats, current_row, current_col);	
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