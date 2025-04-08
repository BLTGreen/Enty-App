use async_graphql::{Context, Object, Result, SimpleObject};

use crate::db::Neo4jGraph;

use neo4rs::query;

use tokio_stream::StreamExt;

#[derive(SimpleObject)]

pub struct Person {

    id: String,

    name: String,

    born: Int,

}

#[derive(Default)]

pub struct SubgraphPersonQuery;

#[Object]

impl SubgraphPersonQuery {

    async fn person(&self, ctx: &Context<'_>, id: String) -> Result<Person> {

        let graph = ctx.data::<Neo4jGraph>()?.lock().await;

        let mut result = graph

            .execute(query("MATCH (p:Person {id: $id}) RETURN p").param("id", id))

            .await?;

        if let Ok(Some(row)) = result.next().await {

            let node = row.get::<neo4rs::Node>("p").unwrap();

            let person = Person {

                id: node.get("id").unwrap(),

                name: node.get("name").unwrap(),

                born: node.get("born").unwrap(),

            };

            Ok(person)

        } else {

            Err("Person not found".into())

        }

    }

}