use std::string::String;
use std::collections::HashMap;
use reqwest::header::ToStrError;

use coercion::{RubyValue, RubyHashKey, hash_key};

pub fn build_response(mut response: reqwest::Response) -> HashMap<RubyHashKey, RubyValue> {
    let mut result: HashMap<RubyHashKey, RubyValue> = HashMap::new();
    result.insert(
        hash_key("status"),
        RubyValue::String(response.status().to_string())
    );
    result.insert(
        hash_key("body"),
        RubyValue::String(String::from(
            match response.text() {
                Ok(val) => val.to_string(),
                Err(err) => String::from(err.to_string())
            }
        ))
    );
    result.insert(
        hash_key("headers"),
        RubyValue::Hash(headers_as_hash(response.headers()))
    );
    return result;
}

fn headers_as_hash(headers: &reqwest::header::HeaderMap) -> HashMap<RubyHashKey, RubyValue> {
        let mut headers_hash: HashMap<RubyHashKey, RubyValue> = HashMap::new();
        for (k,v) in headers.iter() {
            headers_hash.insert(
                RubyHashKey::String(k.as_str().to_string()),
                RubyValue::String(header_value(v.to_str())
            )
        );
    }
    return headers_hash
}

fn header_value(val: Result<&str, ToStrError>) -> String {
    return match val {
        Result::Ok(val) => val.to_string(),
        Result::Err(err) => {
            println!("Error parsing header value. Error: {}", err);
            return String::from("");
        }
    }
}
