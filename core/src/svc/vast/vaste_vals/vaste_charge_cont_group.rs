use std::collections::HashMap;

use crate::{
    def::{ItemGrpId, ItemId},
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

pub struct ValChargeParentGroupFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeParentGroupInfo>,
}

pub struct ValChargeParentGroupInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Parent module group ID.
    pub parent_group_id: ItemGrpId,
    /// Group IDs allowed by charge.
    pub allowed_group_ids: Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_cont_group_fast(&mut self, kfs: &RSet<UItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.charge_cont_group.is_empty(),
            false => self.charge_cont_group.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_cont_group_verbose(
        &mut self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValChargeParentGroupFail> {
        let mut charges = HashMap::new();
        for (&charge_key, &cont_key) in self.charge_cont_group.difference(kfs) {
            charges.insert(
                ctx.u_data.items.id_by_key(charge_key),
                ValChargeParentGroupInfo {
                    parent_item_id: ctx.u_data.items.id_by_key(cont_key),
                    parent_group_id: ctx.u_data.items.get(cont_key).get_group_id().unwrap(),
                    allowed_group_ids: ctx
                        .u_data
                        .items
                        .get(charge_key)
                        .get_axt()
                        .unwrap()
                        .cont_limit
                        .as_ref()
                        .unwrap()
                        .group_ids
                        .clone(),
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeParentGroupFail { charges }),
        }
    }
}
