#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use serde_json::{Map, Value};
use std::io::Cursor;

pub fn convert(input: &str) -> Result<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(Cursor::new(input.as_bytes()));

    let headers = rdr
        .headers()
        .context("CSV must include a header row")?
        .iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let mut out = String::new();
    for record in rdr.records() {
        let record = record.context("failed to read CSV record")?;
        let mut row = Map::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            row.insert(header.clone(), Value::String(value.to_string()));
        }
        let line = serde_json::to_string(&Value::Object(row)).context("failed to encode JSON")?;
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&line);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn convert_csv_with_header_to_json_lines() {
        let input = "name,age\nAlice,30\nBob,25\n";
        let output = convert(input).expect("convert");
        assert_eq!(
            output,
            r#"{"age":"30","name":"Alice"}
{"age":"25","name":"Bob"}"#
        );
    }
}
