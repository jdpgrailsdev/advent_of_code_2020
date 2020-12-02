use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn expense_report() {	
    let input = File::open("./data/day1.data").unwrap();
    let reader = BufReader::new(input);
	
	let expenses: Vec<i64> = reader.lines()
		.map(|l| l.unwrap().parse::<i64>().unwrap())
		.collect();
	
	expenses.iter().for_each(|e| {
		expenses.iter().for_each(|o| {
			if e != o {
				//let sum = compute_expense(*e, *o, false);
				expenses.iter().for_each(|x| {
					if x != e && x != o {
						compute_expense(*e, *o, *x, true);
					}
				});
			}
		});
	});		
}

fn compute_expense(e1: i64, e2: i64, e3:i64, print_output: bool) -> i64 {
	let sum = e1 + e2 + e3;
	let product = e1 * e2 * e3;
	if sum == 2020 && print_output {
		println!("Sum of {:?} and {:?} is 2020, product is {:?}.", e1, e2, product);
	}
	
	return sum;
}