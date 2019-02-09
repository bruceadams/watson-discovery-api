#!/bin/bash

openapi-generator generate \
    -g rust \
    -c openapi-generator-config.json \
    -i https://watson-api-explorer.ng.bluemix.net/listings/discovery-v1.json \
    --reserved-words-mappings match=match_ \
    --reserved-words-mappings return=return_ \
    --reserved-words-mappings type=type_

sed -i~ 's/Item = Value/Item = serde_json::Value/' \
    src/apis/query_modifications_api.rs
