I'm using [async-graphql](https://github.com/async-graphql/async-graphql) and [axum](https://github.com/tokio-rs/axum).

This is a reproduction of the issue: https://github.com/frederikhors/iss-async-graphql-error-handling.

To start:

- `cargo run`

If you open the GraphiQL client at http://localhost:8000, you can use the below query to simulate what I'm trying to understand:

```graphql
mutation {
  mutateWithError
}
```

The backend response is:

```json
{
  "data": null,
  "errors": [
    {
      "message": "I cannot mutate now, sorry!",
      "locations": [/*...*/],
      "path": [
        "mutateWithError"
      ]
    }
  ]
}
```

I like this, but what I don't understand is the tracing part:

```log
2022-09-29T17:01:14.249236Z  INFO async_graphql::graphql:84: close, time.busy: 626µs, time.idle: 14.3µs
  in async_graphql::graphql::parse
  in async_graphql::graphql::request

2022-09-29T17:01:14.252493Z  INFO async_graphql::graphql:108: close, time.busy: 374µs, time.idle: 8.60µs
  in async_graphql::graphql::validation
  in async_graphql::graphql::request

2022-09-29T17:01:14.254592Z  INFO async_graphql::graphql:146: error, error: I cannot mutate now, sorry!
  in async_graphql::graphql::field with path: mutateWithError, parent_type: Mutation, return_type: String!
  in async_graphql::graphql::execute
  in async_graphql::graphql::request

2022-09-29T17:01:14.257389Z  INFO async_graphql::graphql:136: close, time.busy: 2.85ms, time.idle: 30.8µs
  in async_graphql::graphql::field with path: mutateWithError, parent_type: Mutation, return_type: String!
  in async_graphql::graphql::execute
  in async_graphql::graphql::request

2022-09-29T17:01:14.260729Z  INFO async_graphql::graphql:122: close, time.busy: 6.31ms, time.idle: 7.80µs
  in async_graphql::graphql::execute
  in async_graphql::graphql::request

2022-09-29T17:01:14.264606Z  INFO async_graphql::graphql:56: close, time.busy: 16.1ms, time.idle: 22.6µs
  in async_graphql::graphql::request
```

Do you see the `INFO async_graphql::graphql:146: error, error: I cannot mutate now, sorry!`?

1. Why is it an `INFO` event? I would expect it to be an `ERROR` event.

1. And where is the innter error `this is a DB error`? 