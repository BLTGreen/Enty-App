use async_graphql::{Schema, SchemaBuilder, MergedObject};

use crate::schema::subgraph_company::SubgraphCompanyQuery;
use crate::schema::subgraph_med::SubgraphMedQuery;
use crate::schema::subgraph_person::SubgraphPersonQuery;

pub mod subgraph_company;

pub mod subgraph_med;

pub mod subgraph_person;

#[derive(MergedObject, Default)]

pub struct QueryRoot(SubgraphCompanyQuery, SubgraphMedQuery, SubgraphPersonQuery);

pub type AppSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> SchemaBuilder<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription> {

    Schema::build(QueryRoot::default(), async_graphql::EmptyMutation, async_graphql::EmptySubscription)

}