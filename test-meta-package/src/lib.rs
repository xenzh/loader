extern crate config;

extern crate package;
extern crate entity;


use config::Config;

use package::{Id, Package, package};
use entity::{Meta, Kind, Component};


#[derive(Debug)]
struct MetaImpl;
impl Meta for MetaImpl {}


#[derive(Debug)]
struct TestPackage {
    kind: Kind,
    id: Id<Kind>,
    deps: Vec<Id<Kind>>,
}

impl TestPackage {
    fn new() -> TestPackage {
        TestPackage {
            kind: Kind::Meta,
            id: Id::new(Kind::Meta, "test"),
            deps: Vec::new(),
        }
    }
}

impl Package for TestPackage {
    type Kind = Kind;
    type Component = Component;

    fn kind(&self) -> Self::Kind {
        self.kind
    }

    fn id(&self) -> Id<Self::Kind> {
        self.id
    }

    fn depends(&self) -> &[Id<Self::Kind>] {
        &self.deps
    }

    fn build(&self, _cfg: &Config) -> Self::Component {
        Component::Meta(Box::new(MetaImpl {}))
    }
}

fn factory() -> TestPackage {
    TestPackage::new()
}

package!(TestPackage, Kind, Component, factory);
