use std::{
    fmt,
    fs::{OpenOptions, create_dir_all},
    io,
    path::PathBuf,
};

use super::data;
use crate::{
    VERSION,
    util::{Error, move_map_to_arcmap},
};

/// JSON adapted data handler implementation.
///
/// This handler implements persistent cache store in the form of zstd-compressed JSON. When data is
/// loaded, adapted data types are stored in RAM, thus it provides extremely fast access, but has
/// noticeable initialization time and RAM consumption.
pub struct RamJsonAdh {
    folder: PathBuf,
    name: String,
    storage_items: rc::util::RMap<rc::ad::AItemId, rc::ad::ArcItemRt>,
    storage_attrs: rc::util::RMap<rc::ad::AAttrId, rc::ad::ArcAttr>,
    storage_effects: rc::util::RMap<rc::ad::AEffectId, rc::ad::ArcEffectRt>,
    storage_mutas: rc::util::RMap<rc::ad::AItemId, rc::ad::ArcMuta>,
    storage_buffs: rc::util::RMap<rc::ad::ABuffId, rc::ad::ArcBuff>,
    fingerprint: Option<String>,
}
impl RamJsonAdh {
    /// Constructs new handler using path to cache folder and cache file name (without extension).
    pub fn new(folder: PathBuf, name: String) -> Self {
        Self {
            folder,
            name,
            storage_items: rc::util::RMap::new(),
            storage_attrs: rc::util::RMap::new(),
            storage_effects: rc::util::RMap::new(),
            storage_mutas: rc::util::RMap::new(),
            storage_buffs: rc::util::RMap::new(),
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
        move_map_to_arcmap(
            a_data.items.into_values().map(rc::ad::AItemRt::new),
            &mut self.storage_items,
        );
        move_map_to_arcmap(a_data.attrs.into_values(), &mut self.storage_attrs);
        move_map_to_arcmap(
            a_data.effects.into_values().map(rc::ad::AEffectRt::new),
            &mut self.storage_effects,
        );
        move_map_to_arcmap(a_data.mutas.into_values(), &mut self.storage_mutas);
        move_map_to_arcmap(a_data.buffs.into_values(), &mut self.storage_buffs);
        self.fingerprint = Some(fingerprint);
    }
    fn update_persistent_cache(&self, c_data: &data::CData) {
        let full_path = self.get_full_path();
        let file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(full_path)
        {
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
                tracing::error!("unable to write cache file: {e}")
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
    fn get_item(&self, id: &rc::ad::AItemId) -> Option<&rc::ad::ArcItemRt> {
        self.storage_items.get(id)
    }
    fn get_attr(&self, id: &rc::ad::AAttrId) -> Option<&rc::ad::ArcAttr> {
        self.storage_attrs.get(id)
    }
    fn get_effect(&self, id: &rc::ad::AEffectId) -> Option<&rc::ad::ArcEffectRt> {
        self.storage_effects.get(id)
    }
    fn get_mutator(&self, id: &rc::ad::AItemId) -> Option<&rc::ad::ArcMuta> {
        self.storage_mutas.get(id)
    }
    fn get_buff(&self, id: &rc::ad::ABuffId) -> Option<&rc::ad::ArcBuff> {
        self.storage_buffs.get(id)
    }
    fn get_data_fingerprint(&self) -> Option<String> {
        self.fingerprint.clone()
    }
    fn load_cache(&mut self) -> rc::ad::AResult<()> {
        let full_path = self.get_full_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| Error::RamJsonReadFailed(e.to_string()))?;
        let mut raw = Vec::new();
        zstd::stream::copy_decode(file, &mut raw).map_err(|e| Error::RamJsonDecompFailed(e.to_string()))?;
        let c_data =
            serde_json::from_slice::<data::CData>(&raw).map_err(|e| Error::RamJsonParseFailed(e.to_string()))?;
        let (a_data, fingerprint) = c_data.to_adapted();
        self.update_memory_cache(a_data, fingerprint);
        Ok(())
    }
    // Update data in handler with passed data.
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
    fn get_handler_version(&self) -> String {
        VERSION.to_string()
    }
}
