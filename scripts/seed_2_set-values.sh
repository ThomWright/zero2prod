#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
set -o xtrace

SUBSCRIPTIONS_LENGTH=1000
NEWSLETTERS_LENGTH=10
DELIVERIES_LENGTH=5000

TITLE_LENGTH=20
CONTENT_LENGTH=1000

pattern_with_max_length() {
  echo "[a-zA-Z0-9]{0, $1}"
}

mv synth/subscriptions.json synth/subscriptions.json.tmp
jq ".length=$SUBSCRIPTIONS_LENGTH | \

    del(.content.email.pattern) | \
    .content.email.type=\"unique\" | \
    .content.email += {content: {type: \"string\", faker: {generator: \"safe_email\"}}} | \

    del(.content.name.pattern) | \
    .content.name += {faker: {generator: \"name\"}} | \

    .content.subscribed_at += {begin: \"2020-01-01T00:00:00+0000\"} \
    " \
  synth/subscriptions.json.tmp \
  >synth/subscriptions.json

mv synth/newsletters.json synth/newsletters.json.tmp
jq ".length=$NEWSLETTERS_LENGTH | \

  .content.title.pattern=\"$(pattern_with_max_length $TITLE_LENGTH)\" | \
  .content.text_content.pattern=\"$(pattern_with_max_length $CONTENT_LENGTH)\" | \
  .content.html_content.pattern=\"$(pattern_with_max_length $CONTENT_LENGTH)\" | \

  .content.published_at += {begin: \"2020-01-01T00:00:00+0000\"} \
  " \
  synth/newsletters.json.tmp \
  >synth/newsletters.json

mv synth/deliveries.json synth/deliveries.json.tmp
jq ".length=$DELIVERIES_LENGTH" \
  synth/deliveries.json.tmp \
  >synth/deliveries.json

rm ./synth/*.tmp
