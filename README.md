# rust-graphql-with-dataloader

A Rust GraphQL API with [actix-web](https://github.com/actix/actix-web), [async-graphql](https://github.com/async-graphql/async-graphql), [diesel](https://github.com/diesel-rs/diesel) and [dataloader](https://github.com/cksac/dataloader-rs)

## Starting
Just run, and wait for a while since the initial compilation is slow...
```
docker-compose up -d 
```

but after the start the graphql api will be accessible at http://localhost:8080/ in POST
you can access a playground at http://localhost:8080/ GET


## Development

Consider that for running the project in development mode you should use these commands 
for starting the database and the Container for building and starting the application.
### Initial build for the Images.
```
docker-compose build
```
### Initialize the Database
```
docker-compose up -d db
```
### Initialize the API container attaching the bash command
```
docker-compose run api bash
```

## Production 

For running in the production mode just execute the command:
```
docker-compose up -d 
```