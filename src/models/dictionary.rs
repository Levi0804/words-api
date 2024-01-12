use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

use serde_json::from_str;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
	pub dictionary: Vec<String>,
	pub syllables: HashMap<String, u32>,
	pub sn: Vec<String>,
	pub minerals: Vec<String>,
	pub phobias: Vec<String>,
}

impl Dictionary {
	pub fn new() -> Dictionary {
		let dictionary_as_str = fs::read_to_string("english.json").unwrap();
		from_str(&dictionary_as_str).unwrap()
	}

	pub fn solve_query(&self, query: Vec<String>) -> String {
		let cache: HashMap<String, Regex> = query
			.iter()
			.map(|s| (s.clone(), Regex::new(s).unwrap()))
			.collect();
	
		let solves: Vec<String> = self.dictionary.clone()
			.into_iter()
			.filter(|word| {
				query
					.iter()
					.map(|pattern| cache.get(pattern).unwrap())
					.all(|regex| regex.is_match(word))
				})
			.take(15)
			.collect();

		solves.join(" ")
	}
}