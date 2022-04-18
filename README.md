# Zero2prod - email subscription service

## Developing

Run `./scripts/init_db.sh` to set up a database.

Run using `cargo run`.

Or, `cargo run | bunyan` for colourful logs (prerequisite: `cargo install bunyan`).

## Testing

Run `cargo test` for all tests.

Try `TEST_LOG=true cargo test --test integration | bunyan` for integration tests with logs.

## Updating sqlx cache

`cargo sqlx prepare -- --lib`
