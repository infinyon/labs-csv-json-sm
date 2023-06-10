use csv::ReaderBuilder;

use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};
use std::collections::HashMap;
use serde_json::json;

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(record.value.as_ref());
    type CSVRecord = HashMap<String, String>;
    let mut rows = Vec::new();
    for result in reader.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let mut row = HashMap::new();
        let record: CSVRecord = result?;
        for (key, value) in record {
            row.insert(key,value);
        }
        rows.push(row);
    }
    let json = json!(rows);
    let serialized_output = serde_json::to_vec(&json)?;
    Ok((key, RecordData::from(serialized_output)))
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_map() {
        let input = include_str!("../test-data/test.csv");
        let expected = include_str!("../test-data/test.json");
        let result = map(&Record::new(input)).unwrap();

        let expected_value:Value = serde_json::from_str(expected).unwrap();
        let expected_str=(None, RecordData::from(expected_value.to_string()));
        assert_eq!(expected_str, result);
    }
}
