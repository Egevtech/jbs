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
            Value::VString(v) => write!(f, "{:?}", v),
        }
    }
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::VString(v) => write!(f, "{:?}", v),
        }
    }
}

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

pub type Derives = HashMap<String, Value>;

pub trait Operate {
    fn operate(&mut self, derives: &Derives);
}

impl Operate for String {
    fn operate(&mut self, derives: &Derives) {
        if self.len() <= 1 {
            return;
        }

        if self.chars().collect::<Vec<char>>()[0] != '^' {
            return;
        }

        let var_name: &str = &self[1..];

        match derives.get(var_name) {
            Some(val) => {
                val.string(self);
            }
            None => {
                eprintln!("Variable name not found: {var_name}");
                std::process::exit(2);
            }
        }
    }
}

impl Operate for Vec<String> {
    fn operate(&mut self, derives: &Derives) {
        for (index, i) in self.clone().iter().enumerate() {
            if let Some(varname) = i.strip_prefix("^") {
                if let Some(gets) = derives.get(varname) {
                    match gets {
                        Value::String(s) => {
                            self.remove(index);
                            self.push(s.to_string());
                        }
                        Value::VString(v) => {
                            self.remove(index);
                            v.iter().for_each(|f| self.push(f.to_string()));
                        }
                    }
                } else {
                    eprintln!("Variable not found: {}", varname);
                    std::process::exit(2);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_operate_string() {
        let mut str: String = String::from("^ochko");
        let mut derives: Derives = Derives::new();

        derives.insert("ochko".to_string(), Value::String("ZZZ".to_string()));

        str.operate(&derives);

        assert_ne!(str, "^ochko");
        assert_eq!(str, "ZZZ");
    }

    #[test]
    fn test_operate_vector() {
        let mut vec: Vec<String> = vec![String::from("^ochko")];
        let mut derives: Derives = Derives::new();

        derives.insert("ochko".to_string(), Value::String("zz".to_string()));

        vec.operate(&derives);

        assert_ne!(vec, vec!["^ochko"]);
        assert_eq!(vec, vec!["zz"]);
    }
}
