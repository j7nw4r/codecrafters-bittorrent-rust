use serde_json;
use std::env;
use anyhow::bail;
// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let mut encoded_chars = encoded_value.chars();
    // If encoded_value starts with a digit, it's a number
    let Some(encoded_char) = encoded_chars.next() else {
        bail!("Unhandled encoded value: {}", encoded_value)
    };
    
    if encoded_char.is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        Ok(serde_json::Value::String(string.to_string()))
    } else if encoded_char.eq_ignore_ascii_case(&'i')  {
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
        Ok(serde_json::Value::Number(parsed_num.into()))
    } else {
        bail!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value= match decode_bencoded_value(encoded_value) {
            Ok(decoded_value) => decoded_value,
            Err(e) => panic!("{}",e)
        };
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
