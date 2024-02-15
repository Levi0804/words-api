use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::{thread, vec};
use lazy_static::lazy_static;

use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
	pub dictionary: Vec<String>,
	pub syllables: HashMap<String, u32>,
	pub sn: Vec<String>,
	pub minerals: Vec<String>,
	pub phobias: Vec<String>,
}

lazy_static! {
    static ref ENGLISH: Dictionary = Dictionary::new();
}

impl Dictionary {
	pub fn new() -> Self {
		serde_json::from_str(
			&fs::read_to_string("english.json").unwrap()
		)
		.unwrap()
	}
		
	pub fn solve_query(query: &str) -> Vec<String> {
		let length: i32 = ENGLISH.dictionary.len().try_into().unwrap();
		let threads_required = 1; 
		let boundary = 50000;
		let starting_index = Arc::new(Mutex::new(0));
		let ending_index = Arc::new(Mutex::new(boundary));
		let regex = Arc::new(Mutex::new(Regex::new(query).unwrap()));
		let solves = Arc::new(Mutex::new(vec![]));
	
		for thread_number in 0..threads_required {
			let starting_index = Arc::clone(&starting_index);
			let ending_index = Arc::clone(&ending_index);
			let regex = Arc::clone(&regex);
			let solves = Arc::clone(&solves);
	
			let handle = thread::spawn(move || {
				let mut ending_index = ending_index.lock().unwrap();
				let mut starting_index = starting_index.lock().unwrap();
				let pattern = regex.lock().unwrap();
				let mut solves = solves.lock().unwrap();
	
				if thread_number == threads_required - 1 {
					*ending_index = length;
				}
	
				for index in *starting_index..*ending_index {
					if solves.len() == 15 { break; }
					if (*pattern).is_match(ENGLISH.dictionary[index as usize].as_str()) {
						solves.push(&ENGLISH.dictionary[index as usize]);
					}
				}
				*ending_index+=boundary;
				*starting_index+=boundary;
			});
	
			handle.join().unwrap();
		}
	
		let solves = solves.lock().unwrap();
	
		(*solves)
			.iter()
			.cloned()
			.map(|s|s.to_owned())
			.collect()
	 }
	}
