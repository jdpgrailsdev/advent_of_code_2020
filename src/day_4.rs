use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

static REQUIRED: &'static [&str] = &["byr","iyr","eyr","hgt","hcl","ecl","pid",]; //"cid"];

pub fn exercise() {
    let data = load_data();
   // validate_passports(data);
    validate_passport_data(data);
}

fn load_data() -> Vec<String> {
	let input = File::open("./data/day4.data").unwrap();
    let reader = BufReader::new(input);

	let data: Vec<String> = reader.lines()
		.map(|l| { l.unwrap() })
		.collect();

	return data;
}

fn validate_passports(data: Vec<String>) {
    let mut valid = 0;
    let mut set: HashSet<&str> = HashSet::new();

    for passport in &data {
        if passport.trim().is_empty() {
            let count = set.iter()
                .filter(|c| REQUIRED.contains(&c) == true)
                .count();

            if count == REQUIRED.len() {
                valid += 1;
            }

            set.clear();
        } else {
            let creds: Vec<&str> = passport.split(' ')
                .map(|kv| kv.split(':'))
                .map(|mut kv| kv.next().unwrap())
                .collect();
            set.extend(&creds);
        }
    }

    println!("Found {} valid passport(s).", valid);
}

fn validate_passport_data(data: Vec<String>) {
    let mut creds: HashMap<&str,&str> = HashMap::new();
    let mut valid = 0;
    let regexes: HashMap<&str, Regex> = vec![
        ("byr", Regex::new(r"^19[2-9]\d{1}|200[0-2]$").unwrap()),
        ("iyr", Regex::new(r"^201\d{1}|2020$").unwrap()),
        ("eyr", Regex::new(r"^202\d{1}|2030$").unwrap()),
        ("hgt", Regex::new(r"^(:?1[5-8]\d{1}|19[0-3])cm|(:?59|[6-7]\d{1}|7[0-6])in$").unwrap()),
        ("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
        ("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap()),
        ("pid", Regex::new(r"^\d{9}$").unwrap())
    ].into_iter().collect();

    for passport in &data {
        if passport.trim().is_empty() {
            let count = creds.iter()
                .map(|(k,_v)| k)
                .filter(|k| REQUIRED.contains(&k) == true)
                .count();

            if count == REQUIRED.len() {
                let mut is_valid = 0;

                for f in REQUIRED {
                    let v = creds.get(*f).unwrap();
                    let r = regexes.get(f).unwrap();
                    if r.is_match(v) {
                        is_valid += 1;
                    }
                }

                if count == is_valid {
                    valid += 1;
                }
            }

            creds.clear();
        } else {
            let fields = passport.split(' ')
                .map(|kv| kv.split(':'))
                .map(|mut kv| (kv.next().unwrap(), kv.next().unwrap()))
                .collect::<HashMap<&str, &str>>();

            for (k,v) in fields {
                creds.insert(k,v);
            }
        }
    }

    println!("Found {} valid passport(s).", valid);
}