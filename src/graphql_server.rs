use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use crate::schema::AppSchema; // Use your schema module here
use std::sync::Arc;

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn run_graphql_server(schema: Arc<AppSchema>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}