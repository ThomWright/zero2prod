#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

EMAIL_LOCAL_PART=$(openssl rand -hex 12)

curl -v -XPOST \
  -d "email=$EMAIL_LOCAL_PART@example.com&name=XXX" \
  localhost:8000/subscriptions
