use core::str;

use anyhow::Result;
use curl::easy::{Easy, List};
use serde_json::Value;
use colored_json::{Color, ColoredFormatter, PrettyFormatter, Styler};


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

    let f = ColoredFormatter::with_styler(PrettyFormatter::new(), Styler {
        key: Color::Cyan.foreground(),
        string_value: Color::Green.foreground(),
        ..Default::default()
    }, );

    let response_body = str::from_utf8(&data).unwrap();
    let json: Value = serde_json::from_str(response_body).unwrap();
    let colored_response = f.to_colored_json_auto(&json).unwrap();
    println!("{}", colored_response);

    Ok(())
}