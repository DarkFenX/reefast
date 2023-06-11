use std::{collections::HashMap, fmt};

use crate::util::{move_vec_to_map, Error, ErrorKind};

/// Adapted data handler implementation without persistence.
///
/// This handler stores everything only in RAM. Access to data is fast, but has noticeable RAM
/// consumption and adapted data has to be rebuilt every time.
pub struct RamOnlyAdh {
    storage_items: HashMap<rc::ReeInt, rc::ad::ArcItem>,
    storage_attrs: HashMap<rc::ReeInt, rc::ad::ArcAttr>,
    storage_effects: HashMap<rc::ReeInt, rc::ad::ArcEffect>,
    storage_mutas: HashMap<rc::ReeInt, rc::ad::ArcMuta>,
    storage_buffs: HashMap<rc::ReeInt, rc::ad::ArcBuff>,
}
impl RamOnlyAdh {
    pub fn new() -> Self {
        Self {
            storage_items: HashMap::new(),
            storage_attrs: HashMap::new(),
            storage_effects: HashMap::new(),
            storage_mutas: HashMap::new(),
            storage_buffs: HashMap::new(),
        }
    }
}
impl fmt::Debug for RamOnlyAdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RamOnlyAdh()")
    }
}
impl rc::ad::AdaptedDataHandler for RamOnlyAdh {
    /// Get adapted item.
    fn get_item(&self, id: &rc::ReeInt) -> Option<rc::ad::ArcItem> {
        self.storage_items.get(&id).cloned()
    }
    /// Get adapted attribute.
    fn get_attr(&self, id: &rc::ReeInt) -> Option<rc::ad::ArcAttr> {
        self.storage_attrs.get(&id).cloned()
    }
    /// Get adapted effect.
    fn get_effect(&self, id: &rc::ReeInt) -> Option<rc::ad::ArcEffect> {
        self.storage_effects.get(&id).cloned()
    }
    /// Get adapted mutaplasmid.
    fn get_muta(&self, id: &rc::ReeInt) -> Option<rc::ad::ArcMuta> {
        self.storage_mutas.get(&id).cloned()
    }
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &rc::ReeInt) -> Option<rc::ad::ArcBuff> {
        self.storage_buffs.get(&id).cloned()
    }
    /// Get adapted data fingerprint.
    ///
    /// Always return None, since it does not persist data and does not store fingerprint.
    fn get_data_fingerprint(&self) -> Option<&str> {
        None
    }
    /// Load cache from persistent storage.
    ///
    /// Will always fail, since this handler does not implement persistent storage.
    fn load_cache(&mut self) -> rc::ad::AResult<()> {
        Err(Error::new(ErrorKind::NoCacheSupport).into())
    }
    /// Update handler with passed adapted data.
    fn update_data(&mut self, a_data: rc::ad::AData, _: String) {
        move_vec_to_map(a_data.items, &mut self.storage_items);
        move_vec_to_map(a_data.attrs, &mut self.storage_attrs);
        move_vec_to_map(a_data.effects, &mut self.storage_effects);
        move_vec_to_map(a_data.mutas, &mut self.storage_mutas);
        move_vec_to_map(a_data.buffs, &mut self.storage_buffs);
    }
}
