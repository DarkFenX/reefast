use crate::{
    defs::{Amount, EEffectId, EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolFighter, SolItemState},
        item_info::SolAutochargeInfo,
    },
    util::StMap,
};

pub struct SolFighterInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolItemState,
    pub amt_override: Option<Amount>,
    pub autocharges: StMap<EEffectId, SolAutochargeInfo>,
}
impl SolFighterInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolItemState,
        amt_override: Option<Amount>,
        autocharges: StMap<EEffectId, SolAutochargeInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
            amt_override,
            autocharges,
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
            sol_fighter.get_state(),
            sol_fighter.get_amt_override(),
            autocharges,
        )
    }
}
