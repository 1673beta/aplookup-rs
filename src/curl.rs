use core::str;

use anyhow::Result;
use nu_ansi_term::Color::Green;
use curl::easy::{Easy, List};
use serde_json::Value;

pub fn aplookup(url: &str) -> Result<()> {
    let mut list = List::new();
    list.append("Accept: application/activity+json").unwrap();

    let mut handle = Easy::new();
    handle.url(url).unwrap();
    handle.http_headers(list).unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response_body = str::from_utf8(&data).unwrap();
    let json: Value = serde_json::from_str(response_body).unwrap();
    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
    let colored_response = Green.paint(pretty_json);
    println!("{}", colored_response);

    Ok(())
}