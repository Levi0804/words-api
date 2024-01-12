use std::env;
mod models;
use crate::models::Dictionary;

fn main() {
    let args = env::args();
	let dictionary = Dictionary::new();
	let congiured_query = config_query(args);
	println!("{:?}", dictionary.solve_query(congiured_query));
}

fn config_query(mut query: env::Args) -> Vec<String> {
	query.next();

	query
		.filter(|frag| !frag.is_empty())
		.collect()
}