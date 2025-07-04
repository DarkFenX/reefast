use std::collections::HashMap;

use crate::{
    ac,
    def::{AttrVal, ItemId, ItemKey},
    svc::{
        SvcCtx,
        vast::{ValCache, VastFitData},
    },
    util::RSet,
};

pub struct ValChargeSizeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeSizeChargeInfo>,
}

pub struct ValChargeSizeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Size attribute value of current charge.
    pub charge_size: Option<AttrVal>,
    /// Size value allowed by module.
    pub allowed_size: AttrVal,
}
impl ValChargeSizeChargeInfo {
    fn from_fail_cache(ctx: SvcCtx, fail_cache: &ValChargeSizeFailCache) -> Self {
        Self {
            parent_item_id: ctx.uad.items.id_by_key(fail_cache.parent_key),
            charge_size: fail_cache.charge_size,
            allowed_size: fail_cache.allowed_size,
        }
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValChargeSizeFailCache {
    pub(in crate::svc::vast) parent_key: ItemKey,
    pub(in crate::svc::vast) charge_key: ItemKey,
    pub(in crate::svc::vast) charge_size: Option<AttrVal>,
    pub(in crate::svc::vast) allowed_size: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_size_fast(&mut self, kfs: &RSet<ItemKey>, ctx: SvcCtx) -> bool {
        for (module_key, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                ValCache::Todo(allowed_size) => match calculate_item_result(ctx, *module_key, *allowed_size) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail_cache) => {
                        let ret_fail = !kfs.contains(&fail_cache.charge_key);
                        cache.fail(fail_cache);
                        if ret_fail {
                            return false;
                        }
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail_cache) => {
                    if !kfs.contains(&fail_cache.charge_key) {
                        return false;
                    }
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValChargeSizeFail> {
        let mut charges = HashMap::new();
        for (module_key, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                ValCache::Todo(allowed_size) => match calculate_item_result(ctx, *module_key, *allowed_size) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail_cache) => {
                        if !kfs.contains(&fail_cache.charge_key) {
                            charges.insert(
                                ctx.uad.items.id_by_key(fail_cache.charge_key),
                                ValChargeSizeChargeInfo::from_fail_cache(ctx, &fail_cache),
                            );
                        }
                        cache.fail(fail_cache);
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail_cache) => {
                    if !kfs.contains(&fail_cache.charge_key) {
                        charges.insert(
                            ctx.uad.items.id_by_key(fail_cache.charge_key),
                            ValChargeSizeChargeInfo::from_fail_cache(ctx, fail_cache),
                        );
                    }
                }
            }
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeSizeFail { charges }),
        }
    }
}

fn calculate_item_result(
    ctx: SvcCtx,
    module_key: ItemKey,
    allowed_size: AttrVal,
) -> ValCache<AttrVal, ValChargeSizeFailCache> {
    let module = ctx.uad.items.get(module_key).get_module().unwrap();
    let charge_key = match module.get_charge_key() {
        Some(charge_key) => charge_key,
        None => return ValCache::Pass(allowed_size),
    };
    let charge_attrs = match ctx.uad.items.get(charge_key).get_a_attrs() {
        Some(charge_attrs) => charge_attrs,
        None => return ValCache::Pass(allowed_size),
    };
    let charge_size = match charge_attrs.get(&ac::attrs::CHARGE_SIZE) {
        Some(charge_size) => *charge_size,
        None => {
            return ValCache::Fail(ValChargeSizeFailCache {
                parent_key: module_key,
                charge_key,
                charge_size: None,
                allowed_size,
            });
        }
    };
    match charge_size == allowed_size {
        true => ValCache::Pass(allowed_size),
        false => ValCache::Fail(ValChargeSizeFailCache {
            parent_key: module_key,
            charge_key,
            charge_size: Some(charge_size),
            allowed_size,
        }),
    }
}
