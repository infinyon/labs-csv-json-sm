# Fluvio SmartModules

This smartmodule converts CSV records to JSON records.

You can test this smartmodule with the following steps:

```bash
$ fluvio cluster start 
$ smdk build
$ smdk load 
$ fluvio topic create csv-json-topic
$ fluvio consume csv-json-topic --smartmodule=csv-json-sm -e delimiter=";" -e header_case=snake 
```

In another terminal:

```bash
$ fluvio produce csv-json-topic -f ./test-data/input.csv --raw
```

# Params

- `delimiter`: The delimiter used in the CSV file. Default is `,`.
- `header_case`: The case of the header. Default is `none`. Possible values are `snake`, `camel`, `none`.

