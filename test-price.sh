#!/usr/bin/env bash

# This is an example payload for getting a price from Pyth.
UPDATE_ARGS='{
    "price_identifier": "27e867f0f4f61076456d1a73b14c7edc1cf5cef4f4d6193a33424288f11bd0f4"
}'

# Feed through jq to get compressed JSON to avoid CLI weirdness.
UPDATE_JSON=$(echo "$UPDATE_ARGS" | jq -c '.' -M)

# Submit.
near contract call-function as-transaction pyth-oracle.testnet get_price json-args "$UPDATE_JSON" prepaid-gas '300 Tgas' attached-deposit '0 NEAR' sign-as pyth-test.testnet network-config testnet sign-with-legacy-keychain send