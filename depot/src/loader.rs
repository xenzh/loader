use std::io::Result as IoResult;
use std::path::{Path, PathBuf};
use std::fs::read_dir;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

use libloading::{Library, Symbol};
use config::{File, Config};
use log::{warn, info, debug};

use package::{Id, Kind, Component, Package, FACTORY};

use crate::error::{Error, Result};


#[derive(Debug)]
pub struct LoadedPackage<K: Kind, C: Component> {
    lib_path: PathBuf,
    cfg_path: Option<PathBuf>,

    lib: Library,
    cfg: Config,

    id: Id<K>,
    pkg: Box<Package<Kind=K, Component=C>>,
}

type Factory<K, C> = unsafe fn() -> *mut Package<Kind=K, Component=C>;

impl<K: Kind, C: Component> LoadedPackage<K, C> {
    fn new(lib_path: PathBuf, cfg_path: PathBuf) -> Result<Self> {
        info!(target: "loader", "Loading library from '{}'", lib_path.display());
        let lib = Library::new(&lib_path)?;

        let mut out_cfg_path = None;
        let mut cfg = Config::new();
        if cfg_path.exists() {
            debug!(target: "loader", "Found config file: '{}'", cfg_path.display());
            match cfg.merge(File::from(cfg_path.as_path())) {
                Ok(_) => {
                    debug!(target: "loader", "Successfully loaded config from '{}'", cfg_path.display());
                    out_cfg_path = Some(cfg_path);
                },
                Err(e) => warn!(target: "loader", "Error loading config from '{}': {}", cfg_path.display(), e),
            }
        }

        debug!(target: "loader", "Importing and invoking factory symbol: '{}'", std::str::from_utf8(FACTORY).unwrap_or("?"));
        let pkg = unsafe { Box::from_raw(lib.get::<Symbol<Factory<K, C>>>(FACTORY)?()) };

        Ok(LoadedPackage {
            lib_path: lib_path,
            cfg_path: out_cfg_path,
            lib: lib,
            cfg: cfg,
            id: pkg.id(),
            pkg: pkg,
        })
    }

    pub fn lib_path(&self) -> &Path {
        &self.lib_path
    }

    pub fn cfg_path(&self) -> Option<&Path> {
        self.cfg_path.as_ref().map(|cp| cp.as_path())
    }

    pub fn config(&self) -> &Config {
        &self.cfg
    }

    pub fn id(&self) -> Id<K> {
        self.id
    }

    pub fn package(&self) -> &Box<Package<Kind=K, Component=C>> {
        &self.pkg
    }
}

impl<K: Kind, C: Component> Drop for LoadedPackage<K, C> {
    fn drop(&mut self) {
        //drop(&self.pkg);
        //drop(&self.lib);
    }
}

impl<K: Kind, C: Component> Display for LoadedPackage<K, C> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} ({}",
            self.id,
            self.lib_path.display())?;

        if let Some(ref p) = self.cfg_path {
            write!(f, ", {}", p.display())?;
        }
        write!(f, ")")
    }
}


pub struct Loader<K: Kind, C: Component> {
    path: PathBuf,
    packages: HashMap<Id<K>, LoadedPackage<K, C>>,
}

impl<K: Kind, C: Component> Loader<K, C> {
    pub fn from_dir(path: &Path) -> Result<Self> {
        // no such file or directory
        Ok(Loader {
            path: path.to_path_buf(),
            packages: HashMap::new(),
        })
    }

    pub fn load(&mut self) -> Result<()> {
        info!(target: "loader", "Loading packages from '{}'", self.path.display());
        self.packages = read_dir(&self.path).map_err(Error::IoError)?
            .filter_map(IoResult::ok)
            .filter(|de| {
                de.file_type().ok().map(|ft| ft.is_file()).unwrap_or(false) &&
                de.path().extension().map(|ex| ex == "so").unwrap_or(false)
            })
            .filter_map(|de| {
                let mut cfg_path = de.path().clone();
                cfg_path.set_extension("toml");
                LoadedPackage::new(de.path(), cfg_path).ok()
            })
            .map(|lp| {
                debug!("Succesfully loaded package: {}", lp);
                (lp.id, lp)
            })
            .collect();

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item=&LoadedPackage<K, C>> {
        self.packages.iter().map(|(_, lp)| lp)
    }

    pub fn get(&self, id: &Id<K>) -> Option<&LoadedPackage<K, C>> {
        self.packages.get(&id)
    }

    //pub fn all(&self, id: Id) -> impl Iterator<Item=&Box<Package>> {
    //    self.packages.iter()
    //        .filter(move |(k, _)| **k == id)
    //        .map(|(_, v)| v)
    //}
}
