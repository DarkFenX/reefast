use std::hash::{BuildHasher, Hash};

use rustc_hash::FxBuildHasher;

use super::map::Map;

pub(crate) type RMapRMap<K1, K2, V> = MapMap<K1, K2, V, FxBuildHasher, FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct MapMap<K1, K2, V, H1, H2> {
    data: Map<K1, Map<K2, V, H2>, H1>,
}
impl<K1, K2, V, H1, H2> MapMap<K1, K2, V, H1, H2>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self { data: Map::new() }
    }
    // Query methods
    pub(crate) fn get_l1(&self, key1: &K1) -> Option<&Map<K2, V, H2>> {
        self.data.get(key1)
    }
    pub(crate) fn get_value(&self, key1: &K1, key2: &K2) -> Option<&V> {
        self.get_l1(key1).and_then(|m1l| m1l.get(key2))
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K1, &Map<K2, V, H2>)> {
        self.data.iter()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &Map<K2, V, H2>> {
        self.data.values()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key1: K1, key2: K2, entry: V) {
        let m1l = self.data.entry(key1).or_insert_with(|| Map::new());
        m1l.insert(key2, entry);
    }
    pub(crate) fn remove_l2(&mut self, key1: &K1, key2: &K2) -> bool {
        // Returns true only if key has been removed
        let need_cleanup = match self.data.get_mut(key1) {
            None => return false,
            Some(m1l) => m1l.remove(key2).is_some() && m1l.is_empty(),
        };
        if need_cleanup {
            self.data.remove(key1);
        }
        need_cleanup
    }
    pub(crate) fn remove_l1(&mut self, key: &K1) -> Option<Map<K2, V, H2>> {
        self.data.remove(key)
    }
}
impl<K1, K2, V, H1, H2> Default for MapMap<K1, K2, V, H1, H2>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
