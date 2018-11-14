use std::result::Result as StdResult;
use std::env;
use libloading::{Library, Symbol};


pub struct Error;
pub type Result<T> = StdResult<T, Error>;


#[derive(Clone, Copy)]
pub enum Kind {
}


pub struct Id {
    pub kind: Kind,
    pub name: &'static str,
}


impl Id {
    pub fn new(kind: Kind, name: &'static str) ->Id {
        Id { kind: kind, name: name }
    }

    pub fn default(kind: Kind) -> Id {
        Id { kind: kind, name: "" }
    }
}


pub trait Component {
    fn kind() -> Kind;
    fn id(&self) -> &Id;
    fn depends() -> &'static [Id];

//    fn new(l: &Loader) -> Result<Self>;
}


// https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
pub struct Loader;

impl Loader {
    pub fn from_dir(path: &str) -> Result<Self> {
        unimplemented!();
    }

    pub fn from_list(files: &[&str]) -> Result<Self> {
        unimplemented!();
    }

    pub fn load() -> Result<()> {
        unimplemented!();
    }

    pub fn get<C: Component>(&self, id: &Id) -> Result<&C> {
        unimplemented!();
    }

    // into_iter() - but think how to return a type / trait->struct
    pub fn all<C: Component>(&self, id: &Id) -> impl Iterator<Item=C> {
        let dummy: Vec<C> = vec![];
        dummy.into_iter()
    }
}
