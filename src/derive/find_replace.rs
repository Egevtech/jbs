use super::*;

impl Value {
    pub fn string(&self, target: &mut String) {
        match self {
            Value::String(s) => *target = s.to_string(),
            Value::VString(_) => {
                eprintln!("Cannot append Vector to String");
                std::process::exit(1);
            }
        }
    }
}
