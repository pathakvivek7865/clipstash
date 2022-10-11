
# ClipStash
To work on the ClipStash project, open up your editor/IDE to the `clipstash` directory. You will then be able to run `cargo check` to check your code, `cargo test` to test your code, or `cargo run --bin name` where `name` is the name of the binary you wish to run.

## Use Rust Version 1.52.0

### ClipStash
The ClipStash project requires additional steps in order to properly build. You will need the `sqlx-cli` tool which can be installed by running

```
cargo install sqlx-cli
```

After installing the tool, you can configure the database for the project by running

```
sqlx database setup
```