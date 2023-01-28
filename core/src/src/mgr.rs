use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use log;

use crate::{
    cg,
    ch::{CHData, CacheHandler},
    dh::DataHandler,
    Error, Result, VERSION,
};

use super::src::Src;

/// Source manager.
///
/// Source manager is a top-level entity which handles everything related to EVE data. It allows
/// other parts of the library to conveniently switch between different data versions (for example,
/// from Tranquility data to Singularity data).
pub struct SrcMgr {
    sources: RwLock<HashMap<String, Arc<Src>>>,
    default: RwLock<Option<Arc<Src>>>,
    locked_aliases: RwLock<HashSet<String>>,
}
impl SrcMgr {
    /// Construct new `SrcMgr`.
    pub fn new() -> SrcMgr {
        SrcMgr {
            sources: RwLock::new(HashMap::new()),
            default: RwLock::new(None),
            locked_aliases: RwLock::new(HashSet::new()),
        }
    }
    /// Add new data source with custom alias, and optionally make it default.
    pub fn add(
        &self,
        alias: &str,
        data_handler: Box<dyn DataHandler>,
        mut cache_handler: Box<dyn CacheHandler>,
        make_default: bool,
    ) -> Result<()> {
        log::info!("adding source with alias \"{}\", default={}", alias, make_default);
        log::info!("using {:?} as data handler", data_handler);
        log::info!("using {:?} as cache handler", cache_handler);
        if self.exists(alias) {
            return Err(Error::new(format!("source with alias \"{}\" already exists", alias)));
        }
        self.lock_alias(alias);
        let dv = get_data_version(&data_handler);
        if need_cache_regen(dv.clone(), &mut cache_handler) {
            let ch_data = regen_cache(&data_handler).map_err(|e| {
                self.unlock_alias(alias);
                e
            })?;
            update_cache(dv, &mut cache_handler, ch_data);
        }
        self.add_source(alias, cache_handler, make_default);
        self.unlock_alias(alias);
        Ok(())
    }
    /// Remove data source which is stored against passed alias.
    pub fn del(&self, alias: &str) -> Result<()> {
        log::info!("removing source with alias \"{}\"", alias);
        self.sources
            .write()
            .unwrap()
            .remove(alias)
            .ok_or(Error::new(format!("no source with alias \"{}\"", alias)))?;
        match self.default.read().unwrap().as_ref() {
            Some(s) if s.alias == alias => *self.default.write().unwrap() = None,
            _ => (),
        };
        Ok(())
    }
    // Crate methods
    pub(crate) fn get(&self, alias: &str) -> Option<Arc<Src>> {
        self.sources.read().unwrap().get(alias).cloned()
    }
    pub(crate) fn get_default(&self) -> Option<Arc<Src>> {
        self.default.read().unwrap().clone()
    }
    // Private methods
    fn exists(&self, alias: &str) -> bool {
        self.sources.read().unwrap().contains_key(alias) || self.locked_aliases.read().unwrap().contains(alias)
    }
    fn lock_alias(&self, alias: &str) {
        log::debug!("locking alias \"{}\"", alias);
        self.locked_aliases.write().unwrap().insert(alias.into());
    }
    fn unlock_alias(&self, alias: &str) {
        log::debug!("unlocking alias \"{}\"", alias);
        if !self.locked_aliases.write().unwrap().remove(alias) {
            log::error!("attempt to unlock alias which is not locked")
        }
    }
    fn add_source(&self, alias: &str, cache_handler: Box<dyn CacheHandler>, make_default: bool) {
        let src = Arc::new(Src::new(alias.into(), cache_handler));
        if make_default {
            *self.default.write().unwrap() = Some(src.clone());
        };
        self.sources.write().unwrap().insert(src.alias.clone(), src);
    }
}

fn get_data_version(data_handler: &Box<dyn DataHandler>) -> Option<String> {
    match data_handler.get_version() {
        Ok(dv) => Some(dv),
        Err(e) => {
            log::info!("unable to get data version: {}", e);
            None
        }
    }
}

fn get_data_fingerprint(data_version: &str) -> String {
    format!("{}_{}", data_version, VERSION)
}

fn need_cache_regen(data_version: Option<String>, cache_handler: &mut Box<dyn CacheHandler>) -> bool {
    let mut regen = false;
    // Failure to read version is not fatal, we just always generate cache in this case
    let data_version = match data_version {
        Some(dv) => dv,
        None => return true,
    };
    // Failure to load cache is not fatal as well
    match cache_handler.load_cache() {
        Ok(_) => (),
        Err(e) => {
            log::info!("unable to load cache: {}", e);
            return true;
        }
    }
    let data_fp = get_data_fingerprint(&data_version);
    let cache_fp = cache_handler.get_fingerprint();
    if &data_fp != cache_fp {
        log::info!("fingerprint mismatch: {} data vs {} cache", data_fp, cache_fp);
        return true;
    };
    false
}

fn regen_cache(data_handler: &Box<dyn DataHandler>) -> Result<CHData> {
    log::info!("regenerating cache...");
    // If we have to regenerate cache, failure to generate one is fatal
    cg::generate_cache(data_handler.as_ref()).map_err(|e| Error::new(format!("failed to generate cache: {}", e)))
}

fn update_cache(data_version: Option<String>, cache_handler: &mut Box<dyn CacheHandler>, ch_data: CHData) {
    let data_version = data_version.unwrap_or("none".into());
    let data_fp = get_data_fingerprint(&data_version);
    cache_handler.update_cache(ch_data, data_fp)
}
