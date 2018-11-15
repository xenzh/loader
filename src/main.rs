extern crate config;
extern crate log;
extern crate simple_logger;

extern crate package;
extern crate depot;
extern crate entity;


use config::{Config, File};
use log::info;

use package::Id;
use depot::depot::Depot;
use entity::{Kind, Component};


fn main() {
    simple_logger::init().unwrap();

    info!(target: "main", "Loading config");
    let mut config = Config::new();
    config.merge(File::with_name("./Depot.toml")).unwrap();

    let depot = Depot::<Kind, Component>::new(config).unwrap();
    let ld = depot.loader();


    let tc = ld.get(&Id::new(Kind::Meta, "test")).unwrap();
    info!(target: "main", "Loaded: {:?}", tc);
}
