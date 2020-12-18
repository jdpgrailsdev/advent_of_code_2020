use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

pub fn exercise() {
	let mask_regex = Regex::new(r"^mask = ([01: X]+)$").unwrap();
	let memory_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
	let mut memory_map: HashMap<i32,String> = HashMap::new();
	
    let data = load_data();
	let mut mask = String::from("");

	for instruction in data {
		if mask_regex.is_match(&instruction) {
			for captures in mask_regex.captures_iter(&instruction) {
				mask = (&captures[1]).to_string();
			}
			println!("Bit mask is {}.", mask);
		} else {
			for captures in memory_regex.captures_iter(&instruction) {
				let memory_index: i32 = captures[1].to_owned().parse().unwrap();
				let mut memory_value: i32 = captures[2].to_owned().parse().unwrap();
				let mut masked_value = String::from("");
				let mut memory_value_str: String = format!("{:b}", memory_value);
						
				while memory_value_str.len() < mask.len() {
					memory_value_str = "0".to_owned() + &memory_value_str;
				}		
																
				for i in 0..mask.len() {
					let bitmask = mask.chars().nth(i).unwrap();
					let value_bit =  memory_value_str.chars().nth(i).unwrap();
					 
					match bitmask {
						'X' => masked_value.push(value_bit),
						'0' => masked_value.push('0'),
						'1' => masked_value.push('1'),
						_ => println!("Unknown bit mask value {}", bitmask),
					}
				}
				
				memory_map.insert(memory_index, masked_value);
			}
		}
	}
	
	let mut sum: i64 = 0;
	
	for (_key, value) in memory_map {
		sum += isize::from_str_radix(&value, 2).unwrap() as i64;
	}
	
	println!("Initialization value is {}.", sum);
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day14.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}