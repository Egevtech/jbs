pub mod errors;
pub mod pack_executable;
pub mod unpack_executable;

use crate::{
    derive::{Derives, *},
    project::Executable,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ExecutablePack {
    pub name: String,
    pub title: String,

    pub compiler: String,
    pub linker: String,

    pub sources: Vec<String>,

    pub compile_options: Vec<String>,
    pub link_options: Vec<String>,
}

pub trait ToPackedExec {
    fn as_packed(&mut self, derives: Derives) -> ExecutablePack;
}

impl ToPackedExec for Executable {
    fn as_packed(&mut self, derives: Derives) -> ExecutablePack {
        let mut compile_opts: Vec<String> = Vec::new();
        let mut link_opts: Vec<String> = Vec::new();

        if let Some(prj_comp_opts) = &self.compile_options {
            prj_comp_opts
                .iter()
                .for_each(|l| compile_opts.push(l.to_string()));
        }

        if let Some(prj_link_opts) = &self.link_options {
            prj_link_opts
                .iter()
                .for_each(|l| link_opts.push(l.to_string()));
        }
        self.sources.operate(&derives);
        self.sources
            .clone()
            .into_iter()
            .for_each(|s| link_opts.push(format!("build/cache/{}.o", s)));

        self.compiler.operate(&derives);
        self.linker.operate(&derives);

        vec![
            "-o",
            ("build/".to_owned() + &self.executable_canonical_name.clone().unwrap()).as_str(),
        ]
        .into_iter()
        .for_each(|s| link_opts.push(s.to_string()));

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
