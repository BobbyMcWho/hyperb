use helix::{FromRuby, CheckResult, ToRuby, ToRubyResult};
use helix::sys::VALUE;
use std::collections::HashMap;
use std::string::String;

#[derive(Clone,Debug)]
pub enum RubyValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(std::string::String),
    Array(Vec<RubyValue>),
    Object(HashMap<String, RubyValue>)
}

pub enum CheckedRubyValue {
    Null,
    Boolean(<bool as FromRuby>::Checked),
    Integer(<i64 as FromRuby>::Checked),
    Float(f64),
    String(<String as FromRuby>::Checked),
    Array(<Vec<RubyValue> as FromRuby>::Checked),
    Object(<HashMap<String, RubyValue> as FromRuby>::Checked)
}

impl ToRuby for RubyValue {
    fn to_ruby(self) -> ToRubyResult {
        match self {
            RubyValue::Null => ().to_ruby(),
            RubyValue::Boolean(v) => v.to_ruby(),
            RubyValue::Integer(v) => v.to_ruby(),
            RubyValue::Float(v) => v.to_ruby(),
            RubyValue::String(v) => v.to_ruby(),
            RubyValue::Array(v) => v.to_ruby(),
            RubyValue::Object(v) => v.to_ruby(),
        }
    }
}

impl FromRuby for RubyValue {
    type Checked = CheckedRubyValue;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedRubyValue> {
        if let Ok(_) = <()>::from_ruby(value) {
            Ok(CheckedRubyValue::Null)
        } else if let Ok(checked) = bool::from_ruby(value) {
            Ok(CheckedRubyValue::Boolean(checked))
        } else if let Ok(checked) = i64::from_ruby(value) {
            Ok(CheckedRubyValue::Integer(checked))
        } else if let Ok(checked) = f64::from_ruby(value) {
            let float = f64::from_checked(checked);

            if float.is_normal() {
                Ok(CheckedRubyValue::Float(float))
            } else {
                type_error!(format!("Cannot convert {} into an http number", float))
            }
        } else if let Ok(checked) = String::from_ruby(value) {
            Ok(CheckedRubyValue::String(checked))
        } else if let Ok(checked) = Vec::<RubyValue>::from_ruby(value) {
            Ok(CheckedRubyValue::Array(checked))
        } else if let Ok(checked) = HashMap::<String, RubyValue>::from_ruby(value) {
            Ok(CheckedRubyValue::Object(checked))
        } else {
            type_error!(value, "a ruby value")
        }
    }

    fn from_checked(checked: CheckedRubyValue) -> RubyValue {
        match checked {
            CheckedRubyValue::Null => RubyValue::Null,
            CheckedRubyValue::Boolean(c) => RubyValue::Boolean(FromRuby::from_checked(c)),
            CheckedRubyValue::Integer(c) => RubyValue::Integer(FromRuby::from_checked(c)),
            CheckedRubyValue::Float(c) => RubyValue::Float(c),
            CheckedRubyValue::String(c) => RubyValue::String(FromRuby::from_checked(c)),
            CheckedRubyValue::Array(c) => RubyValue::Array(FromRuby::from_checked(c)),
            CheckedRubyValue::Object(c) => RubyValue::Object(FromRuby::from_checked(c))
        }
    }
}
