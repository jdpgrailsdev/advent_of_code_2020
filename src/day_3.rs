use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() {
	let data = load_data();
	let mut total = 1;	
	
	total *= traverse(1,1,(*data).to_vec());
	total *= traverse(3,1,(*data).to_vec());
	total *= traverse(5,1,(*data).to_vec());
	total *= traverse(7,1,(*data).to_vec());
	total *= traverse(1,2,(*data).to_vec());
	
	println!("Product of trees found = {}.", total);
}

fn traverse(index: usize, row: usize, data: Vec<Vec<char>>) -> usize {
	let mut num_trees = 0;
	let mut i = index;
	let mut r = row;
	
	while r < data.len() {
		let t = data.get(r).unwrap();
		
		let pos = t.get(i).unwrap();
		if *pos == '#' {
			num_trees += 1;
		}
		
		r = r + row;
		i = i + index;
		
		while i >= t.len() {
			i = i - t.len();
		}	
	}
	
	println!("Found {:?} trees with {} and {}.", num_trees, index, row);
	return num_trees;
}

fn load_data() -> Vec<Vec<char>> {
	let input = File::open("./data/day3.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<Vec<char>> = reader.lines()
		.map(|l| { l.unwrap().chars().collect() })
		.collect();
		
	return data;
}