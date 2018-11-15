#![feature(decl_macro)]

extern crate config;


use config::Config;

use std::hash::Hash;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};


pub trait Kind: Clone + Copy + Hash + Eq + Debug + Display {}


pub trait Component: Debug {
    // event subscription/notification stuff
}


#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Id<K: Kind> {
    kind: K,
    name: &'static str,
}

impl<K: Kind> Id<K> {
    pub fn new(kind: K, name: &'static str) -> Id<K> {
        Id {kind: kind, name: name}
    }

    pub fn default(kind: K) -> Id<K> {
        Id {kind: kind, name: ""}
    }
}

impl<K: Kind> Display for Id<K> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}-{}", self.kind, self.name)
    }
}


pub trait Package : Debug {
    type Kind: Kind;
    type Component: Component;

    fn kind(&self) -> Self::Kind;
    fn id(&self) -> Id<Self::Kind>;

    fn depends(&self) -> &[Id<Self::Kind>];
    fn build(&self, cfg: &Config) -> Self::Component; // should return a result?
}


pub static FACTORY: &[u8] = b"__create";


pub macro package($pack_type:ty, $kind_type:ty, $comp_type:ty, $ctor:path) {
    #[no_mangle]
    pub extern "C" fn __create() -> *mut $crate::Package<Kind=$kind_type, Component=$comp_type> {
        let ctor: fn() -> $pack_type = $ctor;

        let obj = ctor();
        let boxed: Box<$crate::Package<Kind=$kind_type, Component=$comp_type>> = Box::new(obj);
        Box::into_raw(boxed)
    }
}
