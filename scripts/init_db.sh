#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

usage() {
  cat <<EOF
Initialise a PostgreSQL database to develop against.

Runs PostgreSQL in a Docker container, and runs the migrations against it.

Dependencies:
- Docker
- psql
- sqlx

Optional:
  -h --help                        - Print this help and exit
EOF
}

log() {
  echo -e "${1:-}" >&2
}
logT() {
  echo -e "$(date --utc +'%Y-%m-%dT%H:%M:%SZ') $1" >&2
}

# Check any required dependencies exist
check_environment() {
  local req_commands="sqlx psql docker" # space-separated list
  for comm in $req_commands; do
    if ! command -v "$comm" &>/dev/null; then
      log "🙈 Required command '$comm' could not be found"
      if [[ "$comm" == "sqlx" ]]; then
        log "Use:"
        log "cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
        log "to install it."
      fi
      exit 1
    fi
  done
}

run() {
  local DB_USER=${POSTGRES_USER:=postgres}
  local DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
  local DB_NAME="${POSTGRES_DB:=newsletter}"
  local DB_PORT="${POSTGRES_PORT:=5432}"

  export PGPASSWORD="${DB_PASSWORD}"
  if ! psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' 2>/dev/null; then
    docker run \
      --env POSTGRES_USER=${DB_USER} \
      --env POSTGRES_PASSWORD=${DB_PASSWORD} \
      --env POSTGRES_DB=${DB_NAME} \
      --publish "${DB_PORT}":5432 \
      --detach \
      --name zero2prod_pg \
      postgres:14 \
      postgres -N 1000

    until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' 2>/dev/null; do
      log "Postgres is still unavailable - sleeping"
      sleep 1
    done
    log "Postgres is running on port ${DB_PORT}"
  else
    log "Using existing database on port ${DB_PORT}"
  fi

  log "Running migrations"
  export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
  sqlx database create
  sqlx migrate run

  log "Migrations run, ready to go!"
}

check_environment
run
