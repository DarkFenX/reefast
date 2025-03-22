use itertools::Itertools;

use crate::{
    consts,
    defs::{SkillLevel, SolItemId},
    sol::{svc::vast::SolVastFitData, uad::fit::SolFit},
    util::StSet,
};

pub struct SolValOverloadSkillFail {
    pub td_lvl: Option<SkillLevel>,
    pub items: Vec<SolValOverloadSkillItemInfo>,
}

pub struct SolValOverloadSkillItemInfo {
    pub item_id: SolItemId,
    pub req_lvl: SkillLevel,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_overload_skill_fast(&self, kfs: &StSet<SolItemId>, fit: &SolFit) -> bool {
        if self.overload_td_lvl.is_empty() {
            return true;
        }
        let td_lvl = match fit.skills.get(&consts::items::THERMODYNAMICS) {
            Some(skill) => skill.level,
            None => return self.overload_td_lvl.is_subset(kfs),
        };
        self.overload_td_lvl
            .iter()
            .all(|(item_id, &req_lvl)| td_lvl >= req_lvl || kfs.contains(item_id))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_overload_skill_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        fit: &SolFit,
    ) -> Option<SolValOverloadSkillFail> {
        if self.overload_td_lvl.is_empty() {
            return None;
        }
        let td_lvl = fit.skills.get(&consts::items::THERMODYNAMICS).map(|v| v.level);
        let items = self
            .overload_td_lvl
            .iter()
            .filter(|(item_id, req_lvl)| match td_lvl {
                Some(td_lvl) => **req_lvl > td_lvl,
                None => true,
            } && !kfs.contains(item_id))
            .map(|(&item_id, &req_lvl)| SolValOverloadSkillItemInfo { item_id, req_lvl })
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(SolValOverloadSkillFail { td_lvl, items })
    }
}
