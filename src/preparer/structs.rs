use std::collections::HashMap;
use std::fmt::{Formatter, Debug};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    VString(Vec<String>)
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::String(s) => write!(f, "Value::String({})", s),
            Value::VString(v) => write!(f, "Value::VString({:?})", v),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Project {
    name: String,
    version: String,

    default_target: Option<String>,

    variables: Option<HashMap<String, Value>>,
    executables: HashMap<String, Executable>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Executable {
   pub name: String,

   pub compiler: String,
   pub linker: String,

   pub compile_options: Option<Vec<String>>,
   pub link_options: Option<Vec<String>>,
   pub libraries: Option<Vec<String>>,

   pub sources: Vec<String>,

   /* User does not need to fill fields below */
   pub executable_canonical_name: Option<String>

}
