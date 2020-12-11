use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn exercise() {
    let data = load_data();

	accumulate(data);
}

fn accumulate(data: Vec<String>) {
	let mut accumulator = 0;
	let mut index: i32 = 0;
	let mut more = true;
	let mut executed_instructions: HashSet<i32> = HashSet::new();

	while more {
		if executed_instructions.contains(&index) {
			more = true;
			break;
		} else {
			executed_instructions.insert(index);
		}
		
		let instruction = &data.get(index as usize).unwrap();
		let i = instruction.split(" ").map(String::from).collect::<Vec<String>>();
				
		match i.get(0).unwrap().as_ref() {
			"acc" => index += accumulate_and_advance(i.get(1).unwrap().to_string(), &mut accumulator),
			"jmp" => index += get_next_index(i.get(1).unwrap().to_string()),
			"nop" => index += 1,
			_ => println!("Unexpected instruction {}.", instruction)
		}				
	}
	
	println!("Accumulator contains {}.", accumulator);
}

fn accumulate_and_advance(amount: String, accumulator: &mut i32) -> i32 {
	let amount: i32 = amount.parse().unwrap();
	*accumulator += amount;
	return 1;
}

fn get_next_index(jump_amount: String) -> i32 {
	let number: String = jump_amount.chars().skip(1).collect();
	let amt: i32 = number.parse().unwrap();
	
	if jump_amount.chars().next().unwrap() == '+' {
		return amt;
	} else {
		return amt * -1;
	}
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day8.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}