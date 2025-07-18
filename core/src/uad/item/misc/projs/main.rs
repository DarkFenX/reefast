use crate::{def::ItemKey, uad::UadProjRange, util::RMap};

#[derive(Clone)]
pub(crate) struct Projs {
    pub(super) data: RMap<ItemKey, Option<UadProjRange>>,
}
impl Projs {
    pub(in crate::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(crate) fn add(&mut self, item_key: ItemKey, range: Option<UadProjRange>) {
        self.data.insert(item_key, range);
    }
    pub(crate) fn remove(&mut self, item_key: &ItemKey) -> Option<Option<UadProjRange>> {
        self.data.remove(item_key)
    }
    pub(crate) fn contains(&self, item_key: &ItemKey) -> bool {
        self.data.contains_key(item_key)
    }
    pub(crate) fn get(&self, item_key: &ItemKey) -> Option<Option<UadProjRange>> {
        self.data.get(item_key).copied()
    }
    pub(crate) fn get_range_mut(&mut self, item_key: &ItemKey) -> Option<&mut UadProjRange> {
        match self.data.get_mut(item_key) {
            Some(Some(uad_proj_range)) => Some(uad_proj_range),
            _ => None,
        }
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (ItemKey, Option<UadProjRange>)> {
        self.data.iter().map(|(k, v)| (*k, *v))
    }
    pub(crate) fn iter_projectees(&self) -> impl ExactSizeIterator<Item = ItemKey> {
        self.data.keys().copied()
    }
    pub(crate) fn iter_ranges_mut(&mut self) -> impl Iterator<Item = &mut UadProjRange> {
        self.data.values_mut().filter_map(|v| v.as_mut())
    }
    pub(crate) fn iter_projectees_and_ranges(&self) -> impl Iterator<Item = (ItemKey, UadProjRange)> {
        self.data
            .iter()
            .filter_map(|(projectee_key, prange)| prange.map(|prange| (*projectee_key, prange)))
    }
    pub(crate) fn iter_projectees_and_ranges_mut(&mut self) -> impl Iterator<Item = (ItemKey, &mut UadProjRange)> {
        self.data
            .iter_mut()
            .filter_map(|(projectee_key, prange)| prange.as_mut().map(|prange| (*projectee_key, prange)))
    }
}
