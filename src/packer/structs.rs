use serde::{Serialize, Deserialize};
use crate::preparer::structs::{Executable};

#[derive(Deserialize, Serialize, Debug)]
pub struct ExecutablePack {
    pub name: String,
    pub title: String,

    pub compiler: String,
    pub linker: String,

    pub sources: Vec<String>,

    pub compile_options: Vec<String>,
    pub link_options: Vec<String>
}

pub trait to_packed_exec {
    fn as_packed(&self) -> ExecutablePack;
}

impl to_packed_exec for Executable {
    fn as_packed(&self) -> ExecutablePack {
        let mut compile_opts: Vec<String> = Vec::new();
        let mut link_opts: Vec<String> = Vec::new();

        if let Some(prj_comp_opts) = &self.compile_options {
            prj_comp_opts.into_iter().for_each(|l| compile_opts.push(l.to_string()));
        }

        if let Some(prj_link_opts) = &self.link_options {
            prj_link_opts.into_iter().for_each(|l| link_opts.push(l.to_string()));
            self.sources.clone().into_iter().for_each(|s|
                link_opts.push(format!("build/cache/{}.o", s)));
        }

        vec!["-o", self.executable_canonical_name.clone().unwrap().as_str()]
            .into_iter().for_each(|s| {link_opts.push(s.to_string())});

        ExecutablePack {
            name: self.executable_canonical_name.clone().unwrap(),
            title: self.name.clone(),

            compiler: self.compiler.clone(),
            linker: self.linker.clone(),

            sources: self.sources.clone(),

            compile_options: compile_opts,
            link_options: link_opts,

        }
    }
}