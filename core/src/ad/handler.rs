use std::fmt;

use crate::{
    ad::{AData, AResult, ArcAttr, ArcBuff, ArcEffect, ArcItem, ArcMuta},
    defs::{EAttrId, EBuffId, EEffectId, EItemId, EMutaId},
};

/// Adapted data handler interface definition.
///
/// Primary role of an adapted data handler implementation is to provide adapted data by request of
/// the library. Additionally, it can persist adapted types somewhere to avoid regeneration of
/// adapted data on every run.
pub trait AdaptedDataHandler: fmt::Debug + Send + Sync {
    /// Get adapted item.
    fn get_item(&self, id: &EItemId) -> Option<&ArcItem>;
    /// Get adapted attribute.
    fn get_attr(&self, id: &EAttrId) -> Option<&ArcAttr>;
    /// Get adapted effect.
    fn get_effect(&self, id: &EEffectId) -> Option<&ArcEffect>;
    /// Get adapted mutator.
    fn get_muta(&self, id: &EMutaId) -> Option<&ArcMuta>;
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &EBuffId) -> Option<&ArcBuff>;
    /// Get adapted data fingerprint.
    fn get_data_fingerprint(&self) -> Option<&str>;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> AResult<()>;
    /// Update data in handler with passed data.
    fn update_data(&mut self, data: AData, fingerprint: String);
}
