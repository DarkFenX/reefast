use std::collections::hash_map::Entry;

use itertools::Itertools;

use super::calce_shared::{LIMITED_PRECISION_A_ATTR_IDS, get_base_attr_value, get_r_attr};
use crate::{
    ac, ad,
    def::AttrVal,
    misc::SecZone,
    svc::{
        SvcCtx,
        calc::{Calc, CalcAttrVal, ModAccumFast, Modification, ModificationKey},
        err::KeyedItemLoadedError,
    },
    ud::{UItem, UItemKey},
    util::{RMap, round},
};

impl Calc {
    // Query methods
    pub(crate) fn get_item_attr_val_extra_opt_opt(
        &mut self,
        ctx: SvcCtx,
        item_key: Option<UItemKey>,
        a_attr_id: &ad::AAttrId,
    ) -> Option<AttrVal> {
        item_key.and_then(|item_key| self.get_item_attr_val_extra_opt(ctx, item_key, a_attr_id))
    }
    pub(crate) fn get_item_attr_val_extra_opt(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Option<AttrVal> {
        Some(self.get_item_attr_val_full(ctx, item_key, a_attr_id).ok()?.extra)
    }
    pub(crate) fn get_item_attr_val_extra(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<AttrVal, KeyedItemLoadedError> {
        self.get_item_attr_val_full(ctx, item_key, a_attr_id).map(|v| v.extra)
    }
    pub(crate) fn get_item_attr_val_full(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        // Try accessing cached value
        let item_attr_data = match self.attrs.get_item_attr_data(&item_key) {
            Some(item_attr_data) => item_attr_data,
            None => {
                return Err(KeyedItemLoadedError { item_key });
            }
        };
        if let Some(cval) = item_attr_data.values.get(a_attr_id) {
            return Ok(match item_attr_data.postprocs.get(a_attr_id) {
                Some(postprocs) => {
                    let pp_fn = postprocs.fast;
                    pp_fn(self, ctx, item_key, *cval)
                }
                None => *cval,
            });
        }
        // If it is not cached, calculate and cache it
        let mut cval = self.calc_item_attr_val(ctx, item_key, a_attr_id);
        let item_attr_data = self.attrs.get_item_attr_data_mut(&item_key).unwrap();
        item_attr_data.values.insert(*a_attr_id, cval);
        if let Some(postprocs) = item_attr_data.postprocs.get(a_attr_id) {
            let pp_fn = postprocs.fast;
            cval = pp_fn(self, ctx, item_key, cval);
        }
        Ok(cval)
    }
    pub(in crate::svc::calc) fn get_item_attr_val_no_pp(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        let item_attr_data = match self.attrs.get_item_attr_data(&item_key) {
            Some(item_attr_data) => item_attr_data,
            None => {
                return Err(KeyedItemLoadedError { item_key });
            }
        };
        if let Some(cval) = item_attr_data.values.get(a_attr_id) {
            return Ok(*cval);
        };
        let cval = self.calc_item_attr_val(ctx, item_key, a_attr_id);
        self.attrs
            .get_item_attr_data_mut(&item_key)
            .unwrap()
            .values
            .insert(*a_attr_id, cval);
        Ok(cval)
    }
    pub(in crate::svc) fn iter_item_attr_vals(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, CalcAttrVal)> + use<>, KeyedItemLoadedError> {
        let item = ctx.u_data.items.get(item_key);
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let item_attr_data = match self.attrs.get_item_attr_data(&item_key) {
            Some(item_attr_data) => item_attr_data,
            None => return Err(KeyedItemLoadedError { item_key }),
        };
        let pp_attr_ids = item_attr_data.postprocs.keys().copied().collect_vec();
        let mut cvals = item_attr_data.values.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in item.get_attrs().unwrap().keys() {
            if let Entry::Vacant(entry) = cvals.entry(*attr_id) {
                match self.get_item_attr_val_full(ctx, item_key, attr_id) {
                    Ok(v) => entry.insert(v),
                    _ => continue,
                };
            }
        }
        for pp_attr_id in pp_attr_ids {
            if let Some(cval) = cvals.get(&pp_attr_id) {
                let pp_fn = self
                    .attrs
                    .get_item_attr_data(&item_key)
                    .unwrap()
                    .postprocs
                    .get(&pp_attr_id)
                    .unwrap()
                    .fast;
                let cval = pp_fn(self, ctx, item_key, *cval);
                cvals.insert(pp_attr_id, cval);
            }
        }
        Ok(cvals.into_iter())
    }
    // Private methods
    fn iter_modifications(
        &mut self,
        ctx: SvcCtx,
        item_key: &UItemKey,
        item: &UItem,
        a_attr_id: &ad::AAttrId,
    ) -> impl Iterator<Item = Modification> {
        let mut mods = RMap::new();
        for cmod in self
            .std
            .get_mods_for_affectee(item_key, item, a_attr_id, &ctx.u_data.fits)
            .iter()
        {
            let val = match cmod.raw.get_mod_val(self, ctx) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = ctx.u_data.items.get(cmod.raw.affector_espec.item_key);
            let affector_a_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = ModificationKey::from(cmod);
            let modification = Modification {
                op: cmod.raw.op,
                val,
                proj_mult: self.calc_proj_mult(ctx, cmod),
                res_mult: self.calc_resist_mult(ctx, cmod),
                aggr_mode: cmod.raw.aggr_mode,
                affector_a_item_cat_id,
            };
            mods.insert(mod_key, modification);
        }
        mods.into_values()
    }
    fn calc_item_attr_val(&mut self, ctx: SvcCtx, item_key: UItemKey, a_attr_id: &ad::AAttrId) -> CalcAttrVal {
        let item = ctx.u_data.items.get(item_key);
        let r_attr = match ctx.u_data.src.get_attr(a_attr_id) {
            Some(r_attr) => r_attr,
            None => &get_r_attr(*a_attr_id),
        };
        // Get base value
        let base_val = match a_attr_id {
            // Security modifier is a special case - it takes modified value of another attribute as
            // its own base
            &ac::attrs::SECURITY_MODIFIER => {
                let security_a_attr_id = match ctx.u_data.sec_zone {
                    SecZone::HiSec(_) => ac::attrs::HISEC_MODIFIER,
                    SecZone::LowSec(_) => ac::attrs::LOWSEC_MODIFIER,
                    _ => ac::attrs::NULLSEC_MODIFIER,
                };
                // Fetch base value for the generic attribute depending on solar system sec zone,
                // using its base value as a fallback
                match self.get_item_attr_val_full(ctx, item_key, &security_a_attr_id) {
                    Ok(security_full_val) => {
                        // Ensure that change in any a security-specific attribute value triggers
                        // recalculation of generic security attribute value
                        self.deps.add_anonymous(item_key, security_a_attr_id, *a_attr_id);
                        security_full_val.dogma
                    }
                    Err(_) => get_base_attr_value(item, r_attr),
                }
            }
            // Normal attributes
            _ => get_base_attr_value(item, r_attr),
        };
        // Get base value;
        let mut accumulator = ModAccumFast::new();
        for modification in self.iter_modifications(ctx, &item_key, item, a_attr_id) {
            accumulator.add_val(
                modification.val,
                modification.proj_mult,
                modification.res_mult,
                &modification.op,
                r_attr.is_penalizable(),
                &modification.affector_a_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let mut dogma_val = accumulator.apply_dogma_mods(base_val, r_attr.is_hig());
        // Lower value limit
        if let Some(limiter_attr_id) = r_attr.get_min_attr_id()
            && let Ok(limiter_cval) = self.get_item_attr_val_full(ctx, item_key, &limiter_attr_id)
        {
            self.deps.add_anonymous(item_key, limiter_attr_id, *a_attr_id);
            dogma_val = AttrVal::max(dogma_val, limiter_cval.dogma);
        }
        // Upper value limit
        if let Some(limiter_attr_id) = r_attr.get_max_attr_id()
            && let Ok(limiter_cval) = self.get_item_attr_val_full(ctx, item_key, &limiter_attr_id)
        {
            self.deps.add_anonymous(item_key, limiter_attr_id, *a_attr_id);
            dogma_val = AttrVal::min(dogma_val, limiter_cval.dogma);
        }
        if LIMITED_PRECISION_A_ATTR_IDS.contains(a_attr_id) {
            dogma_val = round(dogma_val, 2);
        }
        // Post-dogma calculations
        let extra_val = accumulator.apply_extra_mods(dogma_val, r_attr.is_hig());
        CalcAttrVal {
            base: base_val,
            dogma: dogma_val,
            extra: extra_val,
        }
    }
}
