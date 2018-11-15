extern crate package;

pub mod meta;
pub mod data;
pub mod library;


pub use crate::meta::Meta;
pub use crate::data::Data;
pub use crate::library::Library;

use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Kind {
    Meta,
    Data,
    Library
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match *self {
            Kind::Meta => "meta",
            Kind::Data => "data",
            Kind::Library => "library",
        })
    }
}

impl package::Kind for Kind {}


#[derive(Debug)]
pub enum Component {
    Meta(Box<Meta>),
    Data(Box<Data>),
    Library(Box<Library>),
}

impl package::Component for Component {}
