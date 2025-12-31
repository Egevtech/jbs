pub mod find_replace;
pub mod operate_value;

pub use operate_value::*;

use crate::derive::Value::VString;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    VString(Vec<String>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{:?}", s),
            VString(v) => write!(f, "{:?}", v),
        }
    }
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            VString(v) => write!(f, "{:?}", v),
        }
    }
}

pub type Derives = HashMap<String, Value>;
