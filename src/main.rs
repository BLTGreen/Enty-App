mod schema;
mod db;
mod graphql_server;

use slint::slint;
use db::connect_to_neo4j;
use tokio::task::LocalSet;
use std::sync::Arc;

slint! {
    export { MainWindow } from "ui/app-window.slint";
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), slint::PlatformError> {
    // 1. Initialize Neo4j database connection.
    let neo4j_graph = connect_to_neo4j("bolt://localhost:7687", "neo4j", "password").await;

    // 2. Create and wrap GraphQL schema.
    let schema = Arc::new(schema::create_schema().data(neo4j_graph).finish());

    // 3. Spawn GraphQL server in a background task.
    let local_set = LocalSet::new();

    local_set.spawn_local(async move {
        if let Err(e) = graphql_server::run_graphql_server(schema.clone()).await {
            eprintln!("Uh oh, you did a fucky-wucky. Failed to run GraphQL server: {}", e);
        }
    });

    // 4. Run the Slint GUI.
    local_set.run_until(async {
        let main_window = MainWindow::new()?;
    let _ = main_window.run();
        Result::<(), slint::PlatformError>::Ok(())
    }).await?;
    Ok(())
}