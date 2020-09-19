# actix-start
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> Just me trying out actix web. Initially felt like a bloated mess, but further exploration of features such as the ServiceFactory
> and web scope add some level of modularity I like. Logging implementation is also pretty neat.
> 
> Will continue to further explore axtix and attempt to use microservices like Redis with it and see how it holds up before I give Rocket a trial
>
> ## Observation on returning custom structs in the form of json objects
> You have two options:
> * Implement actix's `Responder` trait to your custom type and simply return your type from the request handler --> Long way (see /api/config.rs)
> * Derive the `Serialize` trait from `serde` and simply return `web::Json<YourStruct>` from the request handler --> Quick way (see /api/test.rs)

## Run Server

```sh
cargo run
```

## Run Tests

```sh
cargo test
```

