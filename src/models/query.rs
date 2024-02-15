use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Query {
	pub query: String
}

impl Query {
	pub fn config(&self) -> Vec<String> {
		self.query
			.as_str()
			.split_whitespace()
			.map(str::to_string)
			.collect()
	}
}

#[derive(Deserialize, Serialize)]
pub struct QueryJson {
	pub result: Vec<String>
}

impl QueryJson {
	pub fn new(v: Vec<String>) -> Self {
		QueryJson {
			result: v
		}
	}
}