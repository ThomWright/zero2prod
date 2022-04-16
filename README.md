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

## Generating data

Uses [Synth](https://www.getsynth.com/) to generate data to seed the database.

The `bytea` type isn't yet supported (which the `_sqlx_migrations` table uses), so to hack around this:

- Connect to the database
- Clone the database schema: `CREATE DATABASE newsletter_synth TEMPLATE newsletter;`
- Drop the migrations table: `DROP TABLE _sqlx_migrations;`

Import: `synth import --from postgres://postgres:password@localhost:5432/newsletter_synth synth`

~Generate: `synth generate --to postgres://postgres:password@localhost:5432/newsletter_synth synth`~

That doesn't work: [bug](https://github.com/getsynth/synth/issues/270).

So:

- `synth generate --to csv:tmp/data.csv synth`

- XXX:

  ```bash
  synth generate --to csv: synth |
    tail -n +5 -f - |
    grep . |
    psql -h "localhost" -U postgres -d newsletter_synth -c 'COPY subscriptions (email, id, name, subscribed_at) FROM STDIN CSV HEADER'
  ```
