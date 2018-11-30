// should probably go into core package
pub mod query;
pub mod eval;

pub mod method;
pub mod action;


use std::fmt::Debug;

use self::method::Method;


pub trait Meta : Debug {
    fn methods(&self) -> &[&Method];
}
