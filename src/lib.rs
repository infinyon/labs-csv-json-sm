use csv::ReaderBuilder;

use fluvio_smartmodule::{smartmodule, Result as FluvioResult, Record, RecordData};
use std::collections::HashMap;
use serde_json::json;

#[smartmodule(map)]
pub fn map(record: &Record) -> FluvioResult<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(record.value.as_ref());
    type CSVRecord = HashMap<String, String>;

    let rows: Vec<CSVRecord> = reader.deserialize().collect::<Result<_, _>>()?;
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
        let input = include_str!("../test-data/input.csv");
        let result = map(&Record::new(input)).unwrap();

        let expected = include_str!("../test-data/output.json");
        let expected_value:Value = serde_json::from_str(expected).unwrap();
        let expected_str=(None, RecordData::from(expected_value.to_string()));
        assert_eq!(expected_str, result);
    }
}
