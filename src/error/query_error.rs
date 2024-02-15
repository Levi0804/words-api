#![allow(unused)]

use derive_more::Display;
use actix_web::{
	http::{header::ContentType,StatusCode},
	ResponseError, HttpResponse
};

#[derive(Display, Debug)]
pub enum QueryError {
	NoQueryFound
}

impl ResponseError for QueryError {
	fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::json())
			.body(self.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match self {
			QueryError::NoQueryFound => StatusCode::NOT_FOUND,
		}
	}
}