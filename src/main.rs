use anyhow::bail;
use async_graphql::{extensions::Tracing, http::GraphiQLSource, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::{get, on, MethodFilter},
    Router, Server,
};
use tower_http::trace::TraceLayer;

mod tracer;

pub struct Query;

#[Object]
impl Query {
    async fn test(&self) -> String {
        "I'm a test".to_owned()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn mutate_with_error(&self) -> async_graphql::Result<String> {
        let new_string = mutate_with_error().await?;

        Ok(new_string)
    }
}

async fn mutate_with_error() -> anyhow::Result<String> {
    // START my other attempts

    // Ok("hello world".to_string())

    // bail!("this is a bad error!")

    // can_i_mutate_on_db().await.map_err(|e| {
    //     tracing::error!("{}", e);

    //     e.context("I cannot mutate now, sorry!")
    // })

    // let str = can_i_mutate_on_db().await?;
    // Ok(str)

    // END my other attempts

    match can_i_mutate_on_db().await {
        Ok(s) => Ok(s),
        Err(err) => Err(err.context("I cannot mutate now, sorry!")),
    }
}

async fn can_i_mutate_on_db() -> anyhow::Result<String> {
    bail!("this is a DB error!")
}

// #[tracing::instrument(skip(schema, req))]
async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    tracer::init();

    let gql_schema = Schema::build(Query, Mutation, EmptySubscription)
        .extension(Tracing)
        .finish();

    let app = Router::new()
        .route(
            "/",
            on(MethodFilter::GET | MethodFilter::POST, graphql_handler),
        )
        .route("/graphiql", get(graphiql))
        .layer(Extension(gql_schema))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Open graphiql at http://localhost:8000/graphiql");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
