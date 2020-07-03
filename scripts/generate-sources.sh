#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="$( cd "$(dirname "$0")" || exit ; pwd -P )"

docker run --rm -v "${SCRIPT_PATH}/../:/local" openapitools/openapi-generator-cli generate \
    -i https://raw.githubusercontent.com/influxdata/influxdb/master/http/swagger.yml \
    -g rust \
    -o /local/tmp_generated/ \
    --library "reqwest"

# Keep only required
cd "${SCRIPT_PATH}"/../tmp_generated/src/models || exit
rm -r $(ls | grep -v "\<write_precision.rs\>\|\<health_check.rs\>\|\<query.rb\>\|\<delete_predicate_request.rb\>")

# Move sources
mv "${SCRIPT_PATH}"/../tmp_generated/src/models/*.rs "${SCRIPT_PATH}"/../src/generated/models

# cleanup
rm -rf "${SCRIPT_PATH}"/../tmp_generated/*

cd "${SCRIPT_PATH}"/../