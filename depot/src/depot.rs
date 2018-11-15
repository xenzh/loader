use std::path::Path;
use std::fs::canonicalize;
use std::collections::HashMap;

use config::Config;
use log::info;

use package::{Kind, Component, Id};

use crate::loader::Loader;
use crate::error::Result;


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub packages_dir: String,
}

// load plugins based on config: location
// create and store components in dependency order
// event emit & accept mechanism
pub struct Depot<K: Kind, C: Component> {
    loader: Loader<K, C>,
    components: HashMap<Id<K>, C>,
}

impl<K: Kind, C: Component> Depot<K, C> {
    pub fn new(cfg: Config) -> Result<Depot<K, C>> {
        let settings: Settings = cfg.try_into()?;
        let dir = canonicalize(Path::new(&settings.packages_dir))?;

        info!(target: "depot", "Initializing depot in '{}'", dir.display());

        let mut loader = Loader::from_dir(dir.as_path())?;
        loader.load()?;

        let components = Self::init(&loader)?;

        Ok(Depot { loader: loader, components: components })
    }

    // for now
    pub fn loader(&self) -> &Loader<K, C> {
        &self.loader
    }

    fn init(l: &Loader<K, C>) -> Result<HashMap<Id<K>, C>> {
        let order = Self::order(l)?;
        Ok(order.iter()
            .map(|id| l.get(id)
                .map(|lp| {
                    let cfg = lp.config();
                    let comp = lp.package().build(cfg);
                    (*id, comp)
                })
            )
            .filter_map(|opt| opt)
            .collect())
    }

    fn order(l: &Loader<K, C>) -> Result<Vec<Id<K>>> {
        // TODO: actual dependency resolving
        let order: Vec<_> = l.iter().map(|lp| lp.id()).collect();
        Ok(order)
    }
}
