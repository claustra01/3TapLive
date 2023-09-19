use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, Schema, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;

use objects::{QuerySchema, query::Query, mutation::Mutation};
mod objects;
mod db;
mod models;
mod token;
mod schema;

async fn index(schema: web::Data<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // db connection pool
  dotenv().ok();
  let pool: db::Pool = db::establish_connection();

  // schema setup
  let schema = Schema::build(Query, Mutation, EmptySubscription)
    .data(pool.clone())
    .finish();

  // run server
  println!("GraphiQL: http://localhost:8080");
  HttpServer::new(move || {
      App::new()
          .app_data(Data::new(schema.to_owned()))
          .service(web::resource("/").guard(guard::Post()).to(index))
          .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}