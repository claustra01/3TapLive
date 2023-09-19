use async_graphql::{Schema, EmptySubscription};

pub mod gql_objects;
pub mod query;
pub mod mutation;
pub type QuerySchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;