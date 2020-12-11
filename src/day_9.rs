use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() {
    let data = load_data();

	find_first_invalid(data);
//	find_encryption_weakness(data);
}

fn find_first_invalid(data: Vec<i64>) {
	let mut not_found = true;
	let mut starting_index = 0;
	let mut current_index = 25;
	let mut current_sum = 0;
	
	while not_found {
		current_sum = *data.get(current_index).unwrap();
		not_found = is_match(data[(starting_index)..(current_index)].to_vec(), current_sum);
		current_index += 1;
		starting_index += 1;
	}

	println!("The first number that does not have a sum in the previous 25 numbers is {}.", current_sum);	
}

fn find_encryption_weakness(data: Vec<i64>) {
	let mut starting_index = 0;
	let mut current_index = 25;
	let mut sequence: Vec<i64> = vec![];
	let expected_sum = 26796446;
	let mut prev_index = 0;
	
	for current_sum in &data {
		if current_index < data.len() {
			let valid = is_match(data[(starting_index)..(current_index)].to_vec(), *current_sum);
			if !valid {
				if current_index == prev_index + 1 {
					sequence.push(*current_sum);
					prev_index = current_index;
				} else {
					let sequence_sum: i64 = sequence.iter().sum();
					if sequence_sum == expected_sum {
						let sum = *sequence.iter().min().unwrap() + *sequence.iter().max().unwrap();
						println!("The sum of the smallest and largest values in the sequence of invalid numbers is {}", sum);
					}
					prev_index = 0;
					sequence.clear();
				}
			} else {
				let sequence_sum: i64 = sequence.iter().sum();
				if sequence_sum == expected_sum {
					let sum = *sequence.iter().min().unwrap() + *sequence.iter().max().unwrap();
					println!("The sum of the smallest and largest values in the sequence of invalid numbers is {}", sum);
				}
				prev_index = 0;
				sequence.clear();
			}
			current_index += 1;
			starting_index += 1;
		}
	}
}

fn is_match(data: Vec<i64>, expected_sum: i64) -> bool {
	let mut is_match: bool = false;
	
	for i in 0..(data.len()) {
		let a = data.get(i).unwrap();
		for j in 0..(data.len()) {
			let b = data.get(j).unwrap();
			if j != i &&  (a + b) == expected_sum {
				is_match = true;
				break;
			}
		}
	}
	
	return is_match;
}

fn load_data() -> Vec<i64> {
	let input = File::open("./data/day9.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<i64> = reader.lines()
		.map(|l| { l.unwrap().parse().unwrap() })
		.collect();

	return data;
}