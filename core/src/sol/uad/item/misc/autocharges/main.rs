use crate::{ad, sol::ItemId, util::StMap};

#[derive(Clone)]
pub(in crate::sol) struct Autocharges {
    data: StMap<ad::AEffectId, ItemId>,
}
impl Autocharges {
    pub(in crate::sol::uad::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn get(&self, a_effect_id: &ad::AEffectId) -> Option<&ItemId> {
        self.data.get(a_effect_id)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (&ad::AEffectId, &ItemId)> {
        self.data.iter()
    }
    pub(in crate::sol) fn values(&self) -> impl ExactSizeIterator<Item = &ItemId> {
        self.data.values()
    }
    pub(in crate::sol) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(in crate::sol) fn set(&mut self, a_effect_id: ad::AEffectId, autocharge_id: ItemId) {
        self.data.insert(a_effect_id, autocharge_id);
    }
    pub(in crate::sol) fn remove(&mut self, a_effect_id: &ad::AEffectId) {
        self.data.remove(a_effect_id);
    }
    pub(in crate::sol) fn clear(&mut self) {
        // Autocharges are supposed to be rarely used, so deallocate whenever map is empty.
        self.data = StMap::new();
    }
}
