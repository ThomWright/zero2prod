#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
set -o xtrace

export PGPASSWORD="password"

rm -rf tmp

disable_trigger_statements=()
copy_statements=()
enable_trigger_statements=()

for path in ./synth/*.json; do
  # collection name
  c=$(basename "$path" .json)

  synth generate --collection "$c" --to csv:tmp/"${c}" synth

  docker cp "tmp/${c}/collection.csv" zero2prod_pg:"/var/lib/postgresql/data/${c}.csv"

  columns=$(head -n 1 "tmp/${c}/collection.csv")

  disable_trigger_statements+=("ALTER TABLE $c DISABLE TRIGGER ALL;")
  copy_statements+=("COPY $c ($columns) FROM '${c}.csv' CSV HEADER;")
  enable_trigger_statements+=("ALTER TABLE $c ENABLE TRIGGER ALL;")
done

psql -h "localhost" -U postgres -d newsletter_synth -c \
  "BEGIN; ${disable_trigger_statements[*]} ${copy_statements[*]} ${enable_trigger_statements[*]} COMMIT;"
