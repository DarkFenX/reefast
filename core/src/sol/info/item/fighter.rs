use crate::{
    defs::{EEffectId, EItemId, SolFitId, SolItemId},
    sol::{
        info::{SolAutochargeInfo, SolProjInfo},
        uad::item::{SolFighter, SolMinionState},
    },
    util::{AdjustableCount, StMap},
};

pub struct SolFighterInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolMinionState,
    pub count: Option<AdjustableCount>,
    pub autocharges: StMap<EEffectId, SolAutochargeInfo>,
    pub projs: Vec<SolProjInfo>,
}
impl SolFighterInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolMinionState,
        count: Option<AdjustableCount>,
        autocharges: StMap<EEffectId, SolAutochargeInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
            count,
            autocharges,
            projs,
        }
    }
    pub(in crate::sol) fn from_fighter_and_autocharges(
        sol_fighter: &SolFighter,
        autocharges: StMap<EEffectId, SolAutochargeInfo>,
    ) -> Self {
        SolFighterInfo::new(
            sol_fighter.get_id(),
            sol_fighter.get_type_id(),
            sol_fighter.get_fit_id(),
            sol_fighter.get_fighter_state(),
            sol_fighter.get_count(),
            autocharges,
            sol_fighter
                .get_projs()
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
