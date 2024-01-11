#![allow(unused)]

use std::{collections::HashMap, fs, env};

use serde::{Serialize, Deserialize};
use serde_json::from_str;
use regex::Regex;
use std::time::Instant;

fn main() {
    let args = env::args();
	println!("{:?}", solve_query(config_query(args)).join(" "));
}

#[derive(Debug, Serialize, Deserialize)]
struct Dictionary {
	dictionary: Vec<String>,
	syllables: HashMap<String, u32>,
	sn: Vec<String>,
	minerals: Vec<String>,
	phobias: Vec<String>,
}

fn parse_json() -> Dictionary {
	let dictionary_as_str = fs::read_to_string("english.json").unwrap();
	from_str(&dictionary_as_str).unwrap()
}

fn config_query(mut query: env::Args) -> Vec<String> {
	query.next();

	query
		.filter(|frag| !frag.is_empty())
		.collect()
}

fn solve_query(query: Vec<String>) -> Vec<String> {
	let dictionary = parse_json();
    let english_dictionary = dictionary.dictionary;
	
	let start_time = Instant::now();

    let cache: HashMap<String, Regex> = query
        .iter()
        .map(|s| (s.clone(), Regex::new(s).unwrap()))
        .collect();

	let solves = english_dictionary
        .into_iter()
        .filter(|word| {
            query
                .iter()
                .map(|pattern| cache.get(pattern).unwrap())
                .all(|regex| regex.is_match(word))
			})
		.take(15)
        .collect();

	let duration = start_time.elapsed();
	println!("Time taken: {:?}", duration);

	solves
}