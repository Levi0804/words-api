use actix_web::{get, HttpServer, App, web::{Path,Json}};

mod data;
use crate::data::Dictionary;
mod models;
use crate::models::{Query, QueryJson};
mod error;
use crate::error::QueryError;

#[get("/search/{query}")]
async fn get_solve(body: Path<Query>) -> Result<Json<QueryJson>, QueryError> {
    let query = body.into_inner().config(); 
    Ok(Json(QueryJson::new(Dictionary::solve_query(query[0].as_str()))))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new( || {
		App::new()
			.service(get_solve) 
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}