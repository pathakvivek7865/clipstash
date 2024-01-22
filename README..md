
# ClipStash
To work on the ClipStash project, open up your editor/IDE to the `clipstash` directory. You will then be able to run `cargo check` to check your code, `cargo test` to test your code, or `cargo run --bin name` where `name` is the name of the binary you wish to run.

### Tested On 
Rust Version 1.74.1
Cargo Version 1.74.1

### Prerequisite
The ClipStash project requires additional steps in order to properly build. You will need the `sqlx-cli` tool which can be installed by running

```
cargo install sqlx-cli
```

After installing the tool, you can configure the database for the project by running

```
sqlx database setup
```

### .env
```
DATABASE_URL="sqlite:data.db"
```

### Run
To run server use command
Default:
```cargo run --bin httpd```
with Args :
```cargo run --bin httpd -- --connection_string <connection_string> -- template_string <template_string>```
where:
```connection_string``` : database connection string.
```template_directory```: path to directory for rendering templates.

To run client app
```cargo run --bin clipclient```