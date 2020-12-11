use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn exercise() {
    let data = load_data();
	let distribution: HashMap<i32,i32> = HashMap::new();
	let built_in_adapter = data.iter().max().unwrap() + 3;
	let mut adapters = data.clone();
	adapters.push(built_in_adapter);

	println!("Built-in adapter is {}", built_in_adapter);

	let distribution = find_adapter_distribution(0, adapters, distribution);
	
	println!("Adapter distribution: {:?}", distribution);
	println!("Product is {}.", distribution.get(&1).unwrap() * distribution.get(&3).unwrap());
}

fn find_adapter_distribution(mut jolts: i32, data: Vec<i32>, mut distribution: HashMap<i32,i32>) -> HashMap<i32, i32> {
	let mut adapters = data;
	let mut current_difference = 0;
	let mut match_found = false;
	
//	println!("Looking at adapters {:?}...", adapters);
	
	while !match_found {
		for adapter in &adapters {
			let difference = adapter - jolts;
//			println!("Adapter {} and jolts {} have a difference of {}.", adapter, jolts, adapter - jolts);
			if difference == current_difference {
				jolts = *adapter;
				if !distribution.contains_key(&difference) {
					distribution.insert(difference, 1);
				} else {
					distribution.insert(difference, distribution.get(&difference).unwrap() + 1);
				}
				adapters.remove(adapters.iter().position(|&d| d == *adapter).unwrap());
//				println!("Adapters now contains {:?}...", adapters);
				match_found = true;
				break;
			} 
		}
		
		if !match_found {
			current_difference += 1;
			if current_difference > 3 {
				match_found = true;
			}
		}
	}
	
	if adapters.len() > 0 {
		return find_adapter_distribution(jolts, adapters, distribution);
	} else {
		return distribution;	
	}
}

fn load_data() -> Vec<i32> {
	let input = File::open("./data/day10.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<i32> = reader.lines()
		.map(|l| { l.unwrap().parse().unwrap() })
		.collect();

	return data;
}