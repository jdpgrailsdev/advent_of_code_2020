use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn exercise() {
    let data = load_data();

//	count_gold_bag_containers(data);
}

fn count_gold_bag_containers(data: Vec<String>) {
	let mut rules: HashMap<String,Vec<(String,i32)>> = HashMap::new();
	let mut set: HashSet<String> = HashSet::new();
	let container = Regex::new(r"^((?:[a-z]+) (?:[a-z]+)) bags contain").unwrap();
	let colors = Regex::new(r"(\d+) ((?:[a-z]+) (?:[a-z]+)) bag[s]{1}").unwrap();
	
	for rule in &data {		
		for cap in container.captures_iter(rule) {		
			let mut allowed_bags: Vec<(String,i32)> = Vec::new();
			for color_cap in colors.captures_iter(rule) {
				let color = color_cap[2].to_string();
				let number = color_cap[1].to_string().parse().unwrap();
				allowed_bags.push((color, number));
			}
			let color = cap[1].to_string();
			rules.insert(color, allowed_bags);
		}		
	}
	
	for (rule, bags) in rules.iter() {
		println!("*******************************");
		println!("Check rule {} with bags {:?}...", rule, bags);
		let results = find_gold_containers(rule.to_string(), bags.to_vec(), rules.clone());
		for r in results {
			set.insert(r);
		}
	}
	
	println!("{:?}", set);
	println!("{} color(s) can hold a gold bag.", set.len());
}

fn find_gold_containers(rule: String, bags: Vec<(String,i32)>, rules: HashMap<String,Vec<(String,i32)>>) -> HashSet<String> {
	let mut containers: HashSet<String> = HashSet::new();
	
	for b in bags.iter() {
		println!("Inspecting bag {}...", b.0);
		if b.0.contains("shiny gold") {
			println!("Rule {} contains 'shiny gold'.", rule);
			containers.insert(rule.to_string());
		} else if rules.contains_key(&b.0) {			
			if rules.get(&b.0).unwrap().len() > 0 {
				println!("Making recursive call for color {} and bags {:?}...", b.0, rules.get(&b.0).unwrap().to_vec());
				let mut color = &b.0;
				let matches = find_gold_containers(color.to_string(), rules.get(&b.0).unwrap().to_vec(), rules.clone());
				for m in matches {
					containers.insert(m);
				}
			} else {
				println!("Rule {} does not have any bags to inspect.", b.0);
			}
		}
	}
	
	return containers;
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day7.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}