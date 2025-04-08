use async_graphql::{Context, Object, Result, SimpleObject};

use crate::db::Neo4jGraph;

use neo4rs::query;

use tokio_stream::StreamExt;

#[derive(SimpleObject)]

pub struct Media {

    id: String,

    title: String,

    language: String,

    length: String,

    genre: String,

    released: Int,

}

#[derive(Default)]

pub struct SubgraphMedQuery;

#[Object]

impl SubgraphMedQuery {

    async fn media(&self, ctx: &Context<'_>, id: String) -> Result<Media> {

        let graph = ctx.data::<Neo4jGraph>()?.lock().await;

        let mut result = graph

            .execute(query("MATCH (m:Media {id: $id}) RETURN m").param("id", id))

            .await?;

        if let Ok(Some(row)) = result.next().await {

            let node = row.get::<neo4rs::Node>("m").unwrap();

            let media = Media {

                id: node.get("id").unwrap(),

                title: node.get("title").unwrap(),

                language: node.get("language").unwrap(),

                length: node.get("length").unwrap(),

                genre: node.get("genre").unwrap(),

                released: node.get("released").unwrap(),

            };

            Ok(movie)

        } else {

            Err("Movie not found".into())

        }

    }

}