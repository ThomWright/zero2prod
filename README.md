# Zero2prod -- email subscription service

## Developing

Run `./scripts/init_db.sh` to set up a database.

Run using `cargo run`.

Or, `cargo run | bunyan` for colourful logs (prerequisite: `cargo install bunyan`).

## Testing

Run `cargo test` for all tests.

Try `TEST_LOG=true cargo test --test integration | bunyan` for integration tests with logs.

## Updating sqlx cache

`cargo sqlx prepare -- --lib`

## Generating data

Uses [Synth](https://www.getsynth.com/) to generate data to seed the database.

Prerequisites:

- synth
- jq

Process:

- `./scripts/init_db.sh`
- `./scripts/seed_1_import.sh`
- `./scripts/seed_2_set-values.sh`
- `./scripts/seed_3_generate.sh`

### Issues

Synth problems I've encountered (which I need to report and/or help fix):

- no support for bytea datatype (in sqlx migrations table)
- no support for composite primary keys
- error when inserting uuids (so I've had to resort to a hacky way of copying CSVs in)
- failing on existing directories (just Deal With It please!)
- failure to detect unique columns

Also, Synth is designed to work by connecting to production-like databases full of data. When connecting to an empty database (like I have locally) then the JSON files it produces describe an empty database, so no data is generated!
