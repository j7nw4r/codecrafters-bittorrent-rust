use anyhow::Context;

pub fn decode(encoded_value: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().context("could not parse number_string into number")?;
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    Ok(vec![serde_json::Value::String(string.to_string())])
}
