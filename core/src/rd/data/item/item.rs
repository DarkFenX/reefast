use crate::{
    ac, ad,
    rd::{REffectKey, RItemAXt, RShipKind},
    util::{GetId, Named, RMap},
};

// Represents an item (or item type, according to EVE terminology).
//
// An item carries alot of info needed to calculate fit attributes, for example base attribute
// values.
pub(crate) struct RItem {
    a_item: ad::AItem,
    // Extra data extracted from adapted item
    axt: RItemAXt,
    ship_kind: Option<RShipKind>,
    has_online_effect: bool,
    takes_turret_hardpoint: bool,
    takes_launcher_hardpoint: bool,
    // Fields which need slab keys to be filled
    effect_datas: RMap<REffectKey, ad::AItemEffectData>,
    defeff_key: Option<REffectKey>,
}
impl RItem {
    pub(in crate::rd) fn new(a_item: ad::AItem) -> Self {
        let axt = RItemAXt::new_initial(&a_item);
        let ship_kind = get_ship_kind(a_item.cat_id, &a_item.srqs);
        let has_online_effect = has_online_effect(&a_item.effect_datas);
        let takes_turret_hardpoint = has_turret_effect(&a_item.effect_datas);
        let takes_launcher_hardpoint = has_launcher_effect(&a_item.effect_datas);
        Self {
            a_item,
            axt,
            ship_kind,
            has_online_effect,
            takes_turret_hardpoint,
            takes_launcher_hardpoint,
            effect_datas: RMap::new(),
            defeff_key: None,
        }
    }
    pub(in crate::rd) fn fill_key_dependents(&mut self, effect_id_key_map: &RMap<ad::AEffectId, REffectKey>) {
        for (a_effect_id, a_effect_data) in self.a_item.effect_datas.iter() {
            if let Some(&effect_key) = effect_id_key_map.get(a_effect_id) {
                self.effect_datas.insert(effect_key, *a_effect_data);
            }
        }
        self.defeff_key = self.a_item.defeff_id.and_then(|v| effect_id_key_map.get(&v).copied());
    }
    // Methods which expose adapted item info
    pub(crate) fn get_group_id(&self) -> ad::AItemGrpId {
        self.a_item.grp_id
    }
    pub(crate) fn get_category_id(&self) -> ad::AItemGrpId {
        self.a_item.cat_id
    }
    pub(crate) fn get_attrs(&self) -> &RMap<ad::AAttrId, ad::AAttrVal> {
        &self.a_item.attrs
    }
    pub(crate) fn get_effect_datas(&self) -> &RMap<REffectKey, ad::AItemEffectData> {
        &self.effect_datas
    }
    pub(crate) fn get_defeff_key(&self) -> Option<REffectKey> {
        self.defeff_key
    }
    pub(crate) fn get_srqs(&self) -> &RMap<ad::AItemId, ad::ASkillLevel> {
        &self.a_item.srqs
    }
    pub(crate) fn get_max_state(&self) -> ad::AState {
        self.a_item.max_state
    }
    pub(crate) fn get_val_fitted_group_id(&self) -> Option<ad::AItemGrpId> {
        self.a_item.val_fitted_group_id
    }
    pub(crate) fn get_val_online_group_id(&self) -> Option<ad::AItemGrpId> {
        self.a_item.val_online_group_id
    }
    pub(crate) fn get_val_active_group_id(&self) -> Option<ad::AItemGrpId> {
        self.a_item.val_active_group_id
    }
    pub(crate) fn is_disallowed_in_wspace(&self) -> bool {
        self.a_item.disallowed_in_wspace
    }
    // Methods which expose info generated during runtime
    pub(crate) fn get_axt(&self) -> &RItemAXt {
        &self.axt
    }
    pub(crate) fn get_ship_kind(&self) -> Option<RShipKind> {
        self.ship_kind
    }
    pub(crate) fn has_online_effect(&self) -> bool {
        self.has_online_effect
    }
    pub(crate) fn takes_turret_hardpoint(&self) -> bool {
        self.takes_turret_hardpoint
    }
    pub(crate) fn takes_launcher_hardpoint(&self) -> bool {
        self.takes_launcher_hardpoint
    }
}
impl GetId<ad::AItemId> for RItem {
    fn get_id(&self) -> ad::AItemId {
        self.a_item.id
    }
}
impl Named for RItem {
    fn get_name() -> &'static str {
        "RItem"
    }
}

pub(super) fn has_online_effect(item_effects: &RMap<ad::AEffectId, ad::AItemEffectData>) -> bool {
    item_effects.contains_key(&ac::effects::ONLINE)
}
pub(super) fn has_turret_effect(item_effects: &RMap<ad::AEffectId, ad::AItemEffectData>) -> bool {
    item_effects.contains_key(&ac::effects::TURRET_FITTED)
}
pub(super) fn has_launcher_effect(item_effects: &RMap<ad::AEffectId, ad::AItemEffectData>) -> bool {
    item_effects.contains_key(&ac::effects::LAUNCHER_FITTED)
}

fn get_ship_kind(item_cat_id: ad::AItemCatId, item_srqs: &RMap<ad::AItemId, ad::ASkillLevel>) -> Option<RShipKind> {
    match item_cat_id {
        ac::itemcats::SHIP => match item_srqs.contains_key(&ac::items::CAPITAL_SHIPS) {
            true => Some(RShipKind::CapitalShip),
            false => Some(RShipKind::Ship),
        },
        ac::itemcats::STRUCTURE => Some(RShipKind::Structure),
        _ => None,
    }
}
