use async_graphql::{Context, Object, Result, SimpleObject};

use crate::db::Neo4jGraph;

use neo4rs::query;

use tokio_stream::StreamExt;

#[derive(SimpleObject)]

pub struct Company {

    id: String,

    name: String,

    created: i32,

}

#[derive(Default)]

pub struct SubgraphCompanyQuery;

#[Object]

impl SubgraphCompanyQuery {

    async fn company(&self, ctx: &Context<'_>, id: String) -> Result<Company> {

        let graph = ctx.data::<Neo4jGraph>()?.lock().await;

        let mut result = graph

            .execute(query("MATCH (c:Company {id: $id}) RETURN c").param("id", id))

            .await?;

        if let Ok(Some(row)) = result.next().await {

            let node = row.get::<neo4rs::Node>("c").unwrap();

            let company = Company {

                id: node.get("id").unwrap(),

                name: node.get("name").unwrap(),

                created: node.get("created").unwrap(),
            };

            Ok(company)

        } else {

            Err("Company not found".into())

        }

    }

}