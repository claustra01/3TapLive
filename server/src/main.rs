use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use tokio::sync::{mpsc::UnboundedReceiver, Mutex};

use objects::{QuerySchema, query::Query, mutation::Mutation, subscription::Subscription};
mod objects;
mod db;
mod models;
mod token;
mod schema;

static RECEIVER: OnceCell<Mutex<UnboundedReceiver<objects::gql_objects::Comment>>> = OnceCell::new();

async fn index(schema: web::Data<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn index_ws(
  schema: web::Data<QuerySchema>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse> {
  GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
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

  // subscription reciever
  let (cmt, rx) = tokio::sync::mpsc::unbounded_channel::<objects::gql_objects::Comment>();
  RECEIVER.set(Mutex::new(rx)).unwrap();

  // schema setup
  let schema = Schema::build(Query, Mutation, Subscription)
    .data(pool.clone())
    .data(cmt)
    .finish();

  // run server
  println!("GraphiQL: http://localhost:8080");
  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(schema.to_owned()))
      .service(web::resource("/")
        .guard(guard::Post())
        .to(index))
      .service(web::resource("/")
        .guard(guard::Get())
        .guard(guard::Header("upgrade", "websocket"))
        .to(index_ws))
      .service(web::resource("/")
        .guard(guard::Get())
        .to(index_graphiql))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}