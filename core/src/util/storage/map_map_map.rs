use std::hash::{BuildHasher, Hash};

use rustc_hash::FxBuildHasher;

use super::{map::Map, map_map::MapMap};

pub(crate) type RMapRMapRMap<K1, K2, K3, V> = MapMapMap<K1, K2, K3, V, FxBuildHasher, FxBuildHasher, FxBuildHasher>;

#[derive(Clone)]
pub(crate) struct MapMapMap<K1, K2, K3, V, H1, H2, H3> {
    data: Map<K1, MapMap<K2, K3, V, H2, H3>, H1>,
}
impl<K1, K2, K3, V, H1, H2, H3> MapMapMap<K1, K2, K3, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    K3: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
    H3: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self { data: Map::new() }
    }
    // Query methods
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K1, &MapMap<K2, K3, V, H2, H3>)> {
        self.data.iter()
    }
    pub(crate) fn get_l1(&self, key1: &K1) -> Option<&MapMap<K2, K3, V, H2, H3>> {
        self.data.get(key1)
    }
    // Modification methods
    pub(crate) fn add_entry(&mut self, key1: K1, key2: K2, key3: K3, entry: V) {
        let m2l = self.data.entry(key1).or_insert_with(|| MapMap::new());
        m2l.add_entry(key2, key3, entry);
    }
    pub(crate) fn remove_l3(&mut self, key1: &K1, key2: &K2, key3: &K3) {
        let need_cleanup = match self.data.get_mut(key1) {
            None => return,
            Some(m2l) => m2l.remove_l2(key2, key3) && m2l.is_empty(),
        };
        if need_cleanup {
            self.data.remove(key1);
        }
    }
}
impl<K1, K2, K3, V, H1, H2, H3> Default for MapMapMap<K1, K2, K3, V, H1, H2, H3>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
    K3: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
    H3: BuildHasher + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
