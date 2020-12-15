use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn exercise() {
    let data = load_data();

	compute_manhattan_distance(data);
}

fn compute_manhattan_distance(data: Vec<String>) {
	let directions = vec!['N','E','S','W'];
	let mut current_direction = 'E';
	let mut east_west_pos = 0;
	let mut north_south_pos = 0;

	for instruction in data {
		let direction = instruction.chars().next().unwrap();
		let amount: i32 = instruction[1..instruction.len()].parse().unwrap();
		
		if direction == 'R' {
			let moves = (amount/90) as usize;
			let current_index = directions.iter().position(|&i| i == current_direction).unwrap();
			let mut next_index = current_index as i32 + moves as i32;
			if next_index >= directions.len() as i32 {
				next_index -= directions.len() as i32;
			}
			current_direction = *directions.get(next_index as usize).unwrap();
		} else if direction == 'L' {
			let moves = (amount/90) as usize;
			let current_index = directions.iter().position(|&i| i == current_direction).unwrap();						
			let mut next_index = current_index as i32 - moves as i32;
			if(next_index < 0) {
				next_index += directions.len() as i32;
			}
			current_direction = *directions.get(next_index as usize).unwrap();				
		} else if direction == 'F' {
			match current_direction {
				'E' => east_west_pos += amount,
				'W' => east_west_pos -= amount,
				'N' => north_south_pos += amount,
				'S' => north_south_pos -= amount,
				 _ => println!("Unknown direction {}", direction),
			} 
		} else {	
			match direction {
				'E' => east_west_pos += amount,
				'W' => east_west_pos -= amount,
				'N' => north_south_pos += amount,
				'S' => north_south_pos -= amount,
				 _ => println!("Unknown direction {}", direction),
			} 
		}
	}
	
	println!("The ship's Manhattan distance is {}.", (east_west_pos.abs() + north_south_pos.abs()));
	
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day12.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}