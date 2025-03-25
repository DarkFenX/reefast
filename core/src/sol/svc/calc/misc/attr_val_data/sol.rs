use crate::{
    sol::{
        ItemId,
        svc::calc::{ItemAttrPostprocs, misc::ItemAttrValData},
        uad::item::Item,
    },
    util::StMap,
};

use super::{
    pp_fighter_count::{FTR_COUNT_ATTR, fighter_count_postproc_fast, fighter_count_postproc_info},
    pp_skill_level::{SKILL_LVL_ATTR, skill_level_postproc_fast, skill_level_postproc_info},
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct AttrValData {
    pub(super) data: StMap<ItemId, ItemAttrValData>,
}
impl AttrValData {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_item_attr_data(&self, item_id: &ItemId) -> Option<&ItemAttrValData> {
        self.data.get(item_id)
    }
    pub(in crate::sol::svc::calc) fn get_item_attr_data_mut(
        &mut self,
        item_id: &ItemId,
    ) -> Option<&mut ItemAttrValData> {
        self.data.get_mut(item_id)
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn item_loaded(&mut self, item: &Item) {
        let mut item_data = ItemAttrValData::new();
        match item {
            Item::Fighter(_) => {
                item_data.postprocs.insert(
                    FTR_COUNT_ATTR,
                    ItemAttrPostprocs::new(fighter_count_postproc_fast, fighter_count_postproc_info),
                );
            }
            Item::Skill(_) => {
                item_data.postprocs.insert(
                    SKILL_LVL_ATTR,
                    ItemAttrPostprocs::new(skill_level_postproc_fast, skill_level_postproc_info),
                );
            }
            _ => (),
        }
        self.data.insert(item.get_item_id(), item_data);
    }
    pub(in crate::sol::svc::calc) fn item_unloaded(&mut self, item_id: &ItemId) {
        self.data.remove(item_id);
    }
}
