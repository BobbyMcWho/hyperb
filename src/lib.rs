#[macro_use]
extern crate helix;
extern crate reqwest;

use std::string::String;
use std::collections::HashMap;

mod coercion;
use coercion::RubyValue;

ruby! {
    class Hyperb {
        def get(path: String) -> HashMap<String, RubyValue> {
            let mut body = reqwest::get(&path).unwrap();
            let mut result: HashMap<String, RubyValue> = HashMap::new();
            result.insert(
                "status".to_string(),
                RubyValue::String(body.status().to_string())
            );
            result.insert("body".to_string(),
                RubyValue::String(body.text().unwrap().to_string())
            );
            return result;
        }
    }
}
