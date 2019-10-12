#[macro_use]
extern crate helix;
extern crate reqwest;

use std::string::String;
use std::collections::HashMap;

mod coercion;
use coercion::{RubyValue, RubyHashKey, hash_key};

mod response_builder;
use response_builder::build_response;

ruby! {
    class Hyperb {
        def get(path: String) -> HashMap<RubyHashKey, RubyValue> {
            match reqwest::get(&path) {
                Ok(val) => build_response(val),
                Err(err) => { // TODO: build_error(err)
                    let mut result: HashMap<RubyHashKey, RubyValue> = HashMap::new();
                    result.insert(
                        hash_key("error"),
                        RubyValue::String(String::from(format!("error reaching server: {}", err)))
                    );
                    return result;
                }
            }
        }
    }
}
