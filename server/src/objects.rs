use async_graphql::Schema;

pub mod gql_objects;
pub mod query;
pub mod mutation;
pub mod subscription;
pub type QuerySchema = Schema<query::Query, mutation::Mutation, subscription::Subscription>;