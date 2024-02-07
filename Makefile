.SILENT:build test-comma test-comma-readable test-semicolon-snake test-semicolon-snake-readable test-semicolon-camel test-semicolon-camel-readable


build:
	smdk build

test-comma: build
	smdk test --file ./test-data/comma/input.csv --raw

test-comma-readable: build
	smdk test --file ./test-data/comma/input.csv --raw | jq

test-semicolon-snake: build
	smdk test -e delimiter=";" -e header_case=snake --file ./test-data/semicolon-snake/input.csv --raw

test-semicolon-snake-readable: build
	smdk test -e delimiter=";" -e header_case=snake --file ./test-data/semicolon-snake/input.csv --raw | tail -n +1 | jq

test-semicolon-camel: build
	smdk test -e delimiter=";" -e header_case=camel --file ./test-data/semicolon-camel/input.csv --raw

test-semicolon-camel-readable: build
	smdk test -e delimiter=";" -e header_case=camel --file ./test-data/semicolon-camel/input.csv --raw | tail -n +1 | jq
