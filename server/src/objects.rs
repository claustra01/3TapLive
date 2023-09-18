use async_graphql::{Schema, EmptySubscription};

pub mod object;
pub mod query;
pub mod mutation;
pub type QuerySchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;