use std::{
    collections::HashMap,
    fmt,
    fs::{create_dir_all, OpenOptions},
    io,
    path::PathBuf,
};

use crate::util::{move_vec_to_map, Error, ErrorKind};

use super::data;

/// JSON adapted data handler implementation.
///
/// This handler implements persistent cache store in the form of zstd-compressed JSON. When data is
/// loaded, adapted data types are stored in RAM, thus it provides extremely fast access, but has
/// noticeable initialization time and RAM consumption.
pub struct RamJsonAdh {
    folder: PathBuf,
    name: String,
    storage_items: HashMap<rc::ItemId, rc::ad::ArcItem>,
    storage_attrs: HashMap<rc::AttrId, rc::ad::ArcAttr>,
    storage_effects: HashMap<rc::EffectId, rc::ad::ArcEffect>,
    storage_mutas: HashMap<rc::MutaId, rc::ad::ArcMuta>,
    storage_buffs: HashMap<rc::BuffId, rc::ad::ArcBuff>,
    fingerprint: Option<String>,
}
impl RamJsonAdh {
    /// Constructs new handler using path to cache folder and cache file name (without extension).
    pub fn new(folder: PathBuf, name: String) -> Self {
        Self {
            folder: folder,
            name: name,
            storage_items: HashMap::new(),
            storage_attrs: HashMap::new(),
            storage_effects: HashMap::new(),
            storage_mutas: HashMap::new(),
            storage_buffs: HashMap::new(),
            fingerprint: None,
        }
    }
    fn get_full_path(&self) -> PathBuf {
        self.folder.join(format!("{}.json.zst", self.name))
    }
    fn create_cache_folder(&self) -> Option<String> {
        match create_dir_all(&self.folder) {
            Ok(_) => None,
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => None,
                    _ => Some(e.to_string()),
                }
            }
        }
    }
    fn update_memory_cache(&mut self, a_data: rc::ad::AData, fingerprint: String) {
        move_vec_to_map(a_data.items, &mut self.storage_items);
        move_vec_to_map(a_data.attrs, &mut self.storage_attrs);
        move_vec_to_map(a_data.effects, &mut self.storage_effects);
        move_vec_to_map(a_data.mutas, &mut self.storage_mutas);
        move_vec_to_map(a_data.buffs, &mut self.storage_buffs);
        self.fingerprint = Some(fingerprint);
    }
    fn update_persistent_cache(&self, c_data: &data::CData) {
        let full_path = self.get_full_path();
        let file = match OpenOptions::new().create(true).write(true).open(full_path) {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("unable to open cache file for writing: {e}");
                return;
            }
        };
        let json = serde_json::json!(&c_data).to_string();
        match zstd::stream::copy_encode(json.as_bytes(), file, 7) {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("unable to write cache file: {e}");
                return;
            }
        };
    }
}
impl fmt::Debug for RamJsonAdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RamJsonAdh(\"{}\")",
            self.get_full_path().to_str().unwrap_or("<error>")
        )
    }
}
impl rc::ad::AdaptedDataHandler for RamJsonAdh {
    /// Get cached item.
    fn get_item(&self, id: &rc::ItemId) -> Option<rc::ad::ArcItem> {
        self.storage_items.get(&id).cloned()
    }
    /// Get cached attribute.
    fn get_attr(&self, id: &rc::AttrId) -> Option<rc::ad::ArcAttr> {
        self.storage_attrs.get(&id).cloned()
    }
    /// Get cached effect.
    fn get_effect(&self, id: &rc::EffectId) -> Option<rc::ad::ArcEffect> {
        self.storage_effects.get(&id).cloned()
    }
    /// Get cached mutaplasmid.
    fn get_muta(&self, id: &rc::MutaId) -> Option<rc::ad::ArcMuta> {
        self.storage_mutas.get(&id).cloned()
    }
    /// Get cached warfare buff.
    fn get_buff(&self, id: &rc::BuffId) -> Option<rc::ad::ArcBuff> {
        self.storage_buffs.get(&id).cloned()
    }
    /// Get cached data fingerprint.
    fn get_data_fingerprint(&self) -> Option<&str> {
        self.fingerprint.as_deref()
    }
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> rc::ad::AResult<()> {
        let full_path = self.get_full_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| Error::new(ErrorKind::RamJsonReadFailed(e.to_string())))?;
        let mut raw = Vec::new();
        zstd::stream::copy_decode(file, &mut raw)
            .map_err(|e| Error::new(ErrorKind::RamJsonDecompFailed(e.to_string())))?;
        let c_data = serde_json::from_slice::<data::CData>(&raw)
            .map_err(|e| Error::new(ErrorKind::RamJsonParseFailed(e.to_string())))?;
        let (a_data, fingerprint) = c_data.to_adapted();
        self.update_memory_cache(a_data, fingerprint);
        Ok(())
    }
    /// Update data in handler with passed data.
    #[tracing::instrument(name = "adh-ramjson-update", level = "trace", skip_all)]
    fn update_data(&mut self, a_data: rc::ad::AData, fingerprint: String) {
        // Update persistent cache
        let c_data = data::CData::from_adapted(&a_data, &fingerprint);
        match self.create_cache_folder() {
            None => self.update_persistent_cache(&c_data),
            Some(msg) => tracing::error!("unable to create cache folder: {msg}"),
        }
        // Update memory cache
        self.update_memory_cache(a_data, fingerprint);
    }
}
