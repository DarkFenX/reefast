//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use smallvec::SmallVec;

use crate::{
    SecZone, ac, ad,
    err::basic::AttrMetaFoundError,
    sol::{
        ItemId, OpInfo,
        svc::calc::{
            AffectorInfo, AttrValInfo, Calc, LoadedItemFoundError, ModAccumInfo, Modification, ModificationInfo,
            ModificationKey,
        },
        uad::{Uad, item::Item},
    },
    util::{RMap, RMapVec, RSet, round},
};

use super::calce_shared::{LIMITED_PRECISION_A_ATTR_IDS, get_base_attr_value};

struct Affection {
    modification: Modification,
    affectors: SmallVec<AffectorInfo, 1>,
}

impl Calc {
    // Query methods
    pub(in crate::sol) fn iter_item_mods(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, Vec<ModificationInfo>)>, LoadedItemFoundError> {
        let mut info_map = RMapVec::new();
        for a_attr_id in self.iter_item_a_attr_ids(uad, item_id)? {
            let mut attr_info = match self.calc_item_attr_info(uad, item_id, &a_attr_id) {
                Ok(attr_info) => attr_info,
                _ => continue,
            };
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(.., |_| true));
            // info_vec.extend(attr_info.filtered_infos.extract_if(.., |_| true));
            if !info_vec.is_empty() {
                info_map.extend_entries(a_attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn iter_item_a_attr_ids(
        &self,
        uad: &Uad,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = ad::AAttrId> + use<>, LoadedItemFoundError> {
        let mut a_attr_ids = RSet::new();
        for attr_id in uad.items.get_by_id(item_id)?.get_a_attrs_err()?.keys() {
            a_attr_ids.insert(*attr_id);
        }
        for attr_id in self.attrs.get_item_attr_data(item_id).unwrap().values.keys() {
            a_attr_ids.insert(*attr_id);
        }
        Ok(a_attr_ids.into_iter())
    }
    fn iter_affections(&mut self, uad: &Uad, item: &Item, a_attr_id: &ad::AAttrId) -> impl Iterator<Item = Affection> {
        let mut affections = RMap::new();
        for modifier in self.std.get_mods_for_affectee(item, a_attr_id, &uad.fits).iter() {
            let val = match modifier.raw.get_mod_val(self, uad) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = uad.items.get_by_id(&modifier.raw.affector_item_id).unwrap();
            let affector_a_item_cat_id = affector_item.get_a_category_id().unwrap();
            let mod_key = ModificationKey::from(modifier);
            let modification = Modification {
                op: modifier.raw.op,
                val,
                res_mult: self.calc_resist_mult(uad, modifier),
                proj_mult: self.calc_proj_mult(uad, modifier),
                aggr_mode: modifier.raw.aggr_mode,
                affector_a_item_cat_id,
            };
            let affection = Affection {
                modification,
                affectors: modifier.raw.get_affector_info(uad),
            };
            affections.insert(mod_key, affection);
        }
        affections.into_values()
    }
    fn calc_item_attr_info(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) -> Result<AttrValInfo, AttrMetaFoundError> {
        let item = uad.items.get_by_id(item_id).unwrap();
        let a_attr = match uad.src.get_a_attr(a_attr_id) {
            Some(a_attr) => a_attr,
            None => return Err(AttrMetaFoundError { attr_id: *a_attr_id }),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute
        // value.
        let base_attr_info = match a_attr_id {
            &ac::attrs::SECURITY_MODIFIER => {
                // Fetch base value for the generic attribute depending on solar system sec zone,
                // using its base value as a fallback
                let security_a_attr_id = match uad.sec_zone {
                    SecZone::HiSec(_) => ac::attrs::HISEC_MODIFIER,
                    SecZone::LowSec(_) => ac::attrs::LOWSEC_MODIFIER,
                    _ => ac::attrs::NULLSEC_MODIFIER,
                };
                match self.get_item_attr_val_full(uad, item_id, &security_a_attr_id) {
                    Ok(security_full_val) => {
                        // Ensure that change in any a security-specific attribute value triggers
                        // recalculation of generic security attribute value
                        self.deps.add_anonymous(*item_id, security_a_attr_id, *a_attr_id);
                        let mut base_attr_info = AttrValInfo::new(security_full_val.dogma);
                        base_attr_info.effective_infos.push(ModificationInfo {
                            // Technically this modification is not pre-assignment, it is base value
                            // overwrite (which later will be overwritten by any other
                            // pre-assignment regardless of its value), but pre-assignment is still
                            // used in info for simplicity. In any EVE scenario there is no
                            // pre-assignment for this attribute
                            op: OpInfo::BaseAssign,
                            initial_val: security_full_val.dogma,
                            range_mult: None,
                            resist_mult: None,
                            stacking_mult: None,
                            applied_val: security_full_val.dogma,
                            affectors: vec![AffectorInfo {
                                item_id: *item_id,
                                attr_id: Some(security_a_attr_id),
                            }],
                        });
                        base_attr_info
                    }
                    Err(_) => AttrValInfo::new(get_base_attr_value(item, a_attr)),
                }
            }
            _ => AttrValInfo::new(get_base_attr_value(item, a_attr)),
        };
        let mut accumulator = ModAccumInfo::new();
        for affection in self.iter_affections(uad, item, a_attr_id) {
            accumulator.add_val(
                affection.modification.val,
                affection.modification.proj_mult,
                affection.modification.res_mult,
                &affection.modification.op,
                a_attr.penalizable,
                &affection.modification.affector_a_item_cat_id,
                &affection.modification.aggr_mode,
                affection.affectors,
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_attr_info, a_attr.hig);
        // Lower value limit
        if let Some(limiter_a_attr_id) = a_attr.min_attr_id {
            if let Ok(limiter_val) = self.get_item_attr_val_full(uad, item_id, &limiter_a_attr_id) {
                self.deps.add_anonymous(*item_id, limiter_a_attr_id, *a_attr_id);
                if limiter_val.dogma > dogma_attr_info.value {
                    dogma_attr_info.value = limiter_val.dogma;
                    dogma_attr_info.effective_infos.push(ModificationInfo {
                        op: OpInfo::MinLimit,
                        initial_val: limiter_val.dogma,
                        range_mult: None,
                        resist_mult: None,
                        stacking_mult: None,
                        applied_val: limiter_val.dogma,
                        affectors: vec![AffectorInfo {
                            item_id: *item_id,
                            attr_id: Some(limiter_a_attr_id),
                        }],
                    })
                }
            }
        }
        // Upper value limit
        if let Some(limiter_a_attr_id) = a_attr.max_attr_id {
            if let Ok(limiter_val) = self.get_item_attr_val_full(uad, item_id, &limiter_a_attr_id) {
                self.deps.add_anonymous(*item_id, limiter_a_attr_id, *a_attr_id);
                if limiter_val.dogma < dogma_attr_info.value {
                    dogma_attr_info.value = limiter_val.dogma;
                    dogma_attr_info.effective_infos.push(ModificationInfo {
                        op: OpInfo::MaxLimit,
                        initial_val: limiter_val.dogma,
                        range_mult: None,
                        resist_mult: None,
                        stacking_mult: None,
                        applied_val: limiter_val.dogma,
                        affectors: vec![AffectorInfo {
                            item_id: *item_id,
                            attr_id: Some(limiter_a_attr_id),
                        }],
                    })
                }
            }
        }
        if LIMITED_PRECISION_A_ATTR_IDS.contains(a_attr_id) {
            dogma_attr_info.value = round(dogma_attr_info.value, 2);
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, a_attr.hig);
        // Custom post-processing functions - since infos are not cached, it's fine to have it here
        let attr_info = match self.attrs.get_item_attr_data(item_id).unwrap().postprocs.get(a_attr_id) {
            Some(postprocs) => {
                let pp_fn = postprocs.info;
                pp_fn(self, uad, item_id, extra_attr_info)
            }
            None => extra_attr_info,
        };
        Ok(attr_info)
    }
}
