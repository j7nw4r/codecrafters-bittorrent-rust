use anyhow::{bail};

pub fn decode(encoded_value: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    let Some(suffix) = encoded_value.strip_prefix("i") else {
        bail!("could not remove i prefix from: {}", encoded_value)
    };
    let parts = suffix.split("e").map(|x| x.to_owned()).collect::<Vec<String>>();
    let Some(num) = parts.first() else {
        bail!("could not split suffix on \"e\"  {}", suffix)
    };
    let Ok(parsed_num) = num.parse::<i64>() else {
        bail!("could not parse num into i64: {}", num)
    };
    Ok(vec![serde_json::Value::Number(parsed_num.into())])}
