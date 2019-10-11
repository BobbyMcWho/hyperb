#[macro_use]
extern crate helix;
extern crate reqwest;

use std::string::String;

ruby! {
    class Hyperb {
        def get() -> String {
            let body = reqwest::get("https://www.rust-lang.org");
            return body.unwrap().text().unwrap();
        }
    }
}