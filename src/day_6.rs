use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn exercise() {
    let data = load_data();

	//count_total_answers(data);
	count_all_answers(data);
}

fn count_total_answers(data: Vec<String>) {
	let mut set: HashSet<char> = HashSet::new();
	let mut count = 0;
	
	for answers in &data {
		if answers.trim().is_empty() {
			count += set.len();
			set.clear();
		} else {
			for a in answers.chars() {
				set.insert(a);
			}
		}
	}
	
	println!("Total 'yes' answers: {}.", count);
}

fn count_all_answers(data: Vec<String>) {
	let mut answer_map: HashMap<char,i32> = HashMap::new();
	let mut group_count = 0;
	let mut answer_count = 0;
	
	for answers in &data {
		if answers.trim().is_empty() {			
			for (key,val) in answer_map.iter() {
				if *val == group_count {
					answer_count += 1;
				}
			}
			
			group_count = 0;
			answer_map.clear();
		} else {
			for a in answers.chars() {
				if answer_map.contains_key(&a) {
					answer_map.insert(a, answer_map.get(&a).unwrap() + 1);
				} else {
					answer_map.insert(a, 1);
				}
				
			}
			group_count += 1;
		}
	}	
	
	println!("Total 'yes' answers for all questions in a group: {}.", answer_count);
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day6.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}