use std::fs::File;
use std::io::{BufRead, BufReader};

struct Password {
	required_char: String,
	minimum: i32,
	maximum: i32,
	password: String
}

pub fn exercise() {
	let data = load_data();
	let mut valid = 0;
	
	for d in data {
// Original validation logic		
//		let c = d.password.matches(&d.required_char).count();
//		if c >= (d.minimum as usize) && c <= (d.maximum  as usize) {
//			valid += 1;
//		}
		let index1 = d.minimum - 1;
		let index2 = d.maximum - 1;
		
		let required_char = d.required_char.chars().next().unwrap();
		let char1 = d.password.chars().nth(index1 as usize).unwrap();
		let char2 = d.password.chars().nth(index2 as usize).unwrap();
		
		let mut count = 0;
		
		if char1 == required_char {
			count += 1;
		}
		
		if char2 == required_char {
			count += 1;
		}
		
		if count == 1 {
			valid += 1;
		}
	}
	
	println!("Found {:?} valid password(s).", valid);
}

fn load_data() -> Vec<Password> {
	let input = File::open("./data/day2.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<Password> = reader.lines()
		.map(|l| {
			let split: Vec<String> = l.unwrap().split(": ").map(String::from).collect::<Vec<String>>();
			let rule: Vec<String> = split.get(0).unwrap().split("-").map(String::from).collect::<Vec<String>>();			
			let minimum: i32 = rule.get(0).unwrap().parse::<i32>().unwrap();
			let subrule: Vec<String> = rule.get(1).unwrap().split_whitespace().map(String::from).collect::<Vec<String>>();
			let maximum: i32 = subrule.get(0).unwrap().parse::<i32>().unwrap();
			let required_char: String = subrule.get(1).unwrap().to_string();
			let password: String = split.get(1).unwrap().to_string();
			return Password { required_char, minimum, maximum, password }
		})
		.collect();
		
	return data;
}