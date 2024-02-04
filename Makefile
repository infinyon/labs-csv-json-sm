build:
	smdk build

test-comma: build
	smdk test --file ./test-data/comma/input.csv --raw

test-comma-readable: build
	smdk test --file ./test-data/comma/input.csv --raw | jq

test-semicolon: build
	smdk test -e delimiter=";" -e header_case=snake --file ./test-data/semicolon/input.csv --raw

test-semicolon-readable: build
	smdk test -e delimiter=";" -e header_case=snake --file ./test-data/semicolon/input.csv --raw | jq


