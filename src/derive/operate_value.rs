use crate::derive::{Derives, Value};

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
