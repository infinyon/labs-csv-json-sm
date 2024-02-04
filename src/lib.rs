use csv::{ReaderBuilder, Trim};
use fluvio_smartmodule::{
    dataplane::smartmodule::SmartModuleExtraParams, smartmodule, Record, RecordData, Result,
};
use heck::{ToLowerCamelCase, ToSnakeCase};
use serde_json::{json, Value};
use std::sync::OnceLock;

static PARAMS: OnceLock<Params> = OnceLock::new();
const DELIMITER_PARAM_NAME: &str = "delimiter";
const HEADER_CASE_PARAM_NAME: &str = "header_case";
const DEFAULT_DELIMITER: u8 = b',';

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let params = PARAMS.get().expect("params is not initialized");
    let key = record.key.clone();

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(params.delimiter)
        .has_headers(true)
        .trim(Trim::All)
        .from_reader(record.value.as_ref());

    let mut rows: Vec<Value> = Vec::new();

    let headers: Vec<String> = csv_reader
        .headers()?
        .iter()
        .map(|h| match params.header_case {
            HeaderCase::Camel => h.to_lower_camel_case(),
            HeaderCase::Snake => h.to_snake_case(),
            HeaderCase::None => h.to_string(),
        })
        .collect();

    for record in csv_reader.records() {
        let json_object: Value = headers
            .iter()
            .zip(record?.iter())
            .map(|(key, value)| (key.clone(), json!(value)))
            .collect();
        rows.push(json_object);
    }

    let serialized_output = serde_json::to_vec(&rows)?;
    Ok((key, RecordData::from(serialized_output)))
}

#[smartmodule(init)]
fn init(params: SmartModuleExtraParams) -> Result<()> {
    let delimiter_param = params
        .get(DELIMITER_PARAM_NAME)
        .map_or(DEFAULT_DELIMITER, |v| {
            v.chars().next().expect("delimiter is empty") as u8
        });

    let case_param = params
        .get(HEADER_CASE_PARAM_NAME)
        .map_or(HeaderCase::None, |v| {
            v.to_string().try_into().unwrap_or_else(|e| {
                panic!("failed to parse header case: {}", e);
            })
        });

    PARAMS
        .set(Params::new(delimiter_param, case_param))
        .expect("params is already initialized");

    Ok(())
}

#[derive(Debug)]
struct Params {
    delimiter: u8,
    header_case: HeaderCase,
}

#[derive(Debug)]
enum HeaderCase {
    Camel,
    Snake,
    None,
}

impl TryFrom<String> for HeaderCase {
    type Error = &'static str;
    fn try_from(s: String) -> core::result::Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "camel" => Ok(HeaderCase::Camel),
            "snake" => Ok(HeaderCase::Snake),
            "" | "none" => Ok(HeaderCase::None),
            _ => Err("Invalid header case"),
        }
    }
}

impl Params {
    fn new(delimiter: u8, header_case: HeaderCase) -> Self {
        Self {
            delimiter,
            header_case,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_map() {
        init(SmartModuleExtraParams::new(
            vec![
                (DELIMITER_PARAM_NAME.to_string(), ",".to_string()),
                (HEADER_CASE_PARAM_NAME.to_string(), "none".to_string()),
            ]
            .into_iter()
            .collect(),
            None,
        ))
        .unwrap();
        let input = include_str!("../test-data/comma/input.csv");
        let result = map(&Record::new(input)).unwrap();

        let expected = include_str!("../test-data/comma/output.json");
        let expected_value: Value = serde_json::from_str(expected).unwrap();
        let expected_str = (None, RecordData::from(expected_value.to_string()));
        assert_eq!(expected_str, result);
    }
}

