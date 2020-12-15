use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() {
    let data = load_data();

	let initial_timestamp: i64 = data.get(0).unwrap().parse().unwrap();
	let mut bus_ids: Vec<i64> = data.get(1).unwrap().split(',').filter(|c| *c != "x").map(|c| c.parse::<i64>().unwrap() as i64).collect();
	bus_ids.sort();
	
	let mut closest_departure = i64::MAX;
	let mut next_bus_id = 0;
	
	for bus_id in bus_ids {
		let next_departure = find_next_departure(bus_id, initial_timestamp, 0);
		if next_departure - initial_timestamp < closest_departure {
			closest_departure = next_departure - initial_timestamp;
			next_bus_id = bus_id;
		}
	}
	
	println!("ID of earliest bus times minutes to wait is {}", next_bus_id * closest_departure);
}

fn find_next_departure(bus_id: i64, timestamp: i64, mut current_timestamp: i64) -> i64 {
	while(current_timestamp < timestamp) {
		current_timestamp = find_next_departure(bus_id, timestamp, current_timestamp + bus_id);
	}
	
	return current_timestamp;
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day13.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}