#![allow(unused)]

use std::{collections::HashMap, fs};

use serde::{Serialize, Deserialize};
use serde_json::from_str;

fn main() {
    let args: StringVec = std::env::args().collect();

	let query = &args[1..];

	solve_query(query);
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

fn solve_query(query: &[String]) {
	let solves: Vec<String> = Vec::new();

	let query_vec = query.split(|e| true);
	
	println!("{:?}", query_vec);

}