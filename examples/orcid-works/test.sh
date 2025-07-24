#!/usr/bin/env bash

# 0) set your ORCID ID or sample ID
ORCID_ID="0000-0002-1825-0097"

# 1) initial fetch
cargo run -p orcid-works-cli -- -i "$ORCID_ID" -o output.json

# 2) mangle the JSON (requires at least 3 records)
jq -f filter.jq output.json > output-modified.json
cp output-modified.json output-modified-1.json
cp output-modified.json output-modified-2.json

# 3) refetch (diff-aware)
cargo run -p orcid-works-cli -- -i "$ORCID_ID" -o output-modified-1.json
# => only missing/changed records are fetched

# 3') force-fetch (bypass diff)
cargo run -p orcid-works-cli -- -i "$ORCID_ID" -o output-modified-2.json --force-fetch
# => all records are fetched

# 4) compare results (expected: identical)
diff -q output.json output-modified-1.json
diff -q output.json output-modified-2.json
