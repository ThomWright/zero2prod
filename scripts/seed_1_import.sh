#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
set -o xtrace

export PGPASSWORD="password"

# The bytea data type isn't supported, so we need to do this hack:

psql -h "localhost" -U postgres -d postgres \
  -c 'DROP DATABASE newsletter_synth;' || true
psql -h "localhost" -U postgres -d postgres \
  -c 'CREATE DATABASE newsletter_synth TEMPLATE newsletter;'
psql -h "localhost" -U postgres -d newsletter_synth \
  -c 'DROP TABLE _sqlx_migrations;'

rm -rf synth

synth import --from postgres://postgres:password@localhost:5432/newsletter_synth synth
