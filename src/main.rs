#![allow(unused)]

use std::{collections::HashMap, fs, env};

use serde::{Serialize, Deserialize};
use serde_json::from_str;
use regex::Regex;

fn main() {
    let args = env::args();
	println!("{:?}", solve_query(config_query(args)));
}

type StringVec = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
struct Dictionary {
	dictionary: StringVec,
	syllables: HashMap<String, u32>,
	sn: StringVec,
	minerals: StringVec,
	phobias: StringVec,
}

fn parse_json() -> Dictionary {
	let dictionary_as_str = fs::read_to_string("english.json").unwrap();
	from_str(&dictionary_as_str).unwrap()
}

fn config_query(mut query: env::Args) -> Vec<String> {
	query.next();

	let mut query_collection: Vec<String> = Vec::new();

	for str in query {
		if !str.is_empty() {
			query_collection.push(str);
		}
	}

	query_collection
	
}

fn solve_query(query: Vec<String>) -> String {
	let dictionary = parse_json();
	let english_dictionary = dictionary.dictionary;
	let mut solves: Vec<String> = Vec::new();

	for word in english_dictionary {
	let word_satisfies = query
		.iter()
		.map(|pattern|Regex::new(pattern).unwrap())
		.all(|regex| regex.is_match(&word));
	if word_satisfies {
		solves.push(word);
		}
	}

	solves.join(" ")
}