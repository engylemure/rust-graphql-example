pub mod context;
pub mod mutation;
pub mod objects;
pub mod query;
pub mod subscription;
pub mod utils;
pub mod input;
pub mod guards;

use async_graphql::Schema as GqlSchema;

pub use mutation::Mutation;
pub use query::QueryRoot;
pub use subscription::Subscription;

pub type Schema = GqlSchema<QueryRoot, Mutation, Subscription>;
