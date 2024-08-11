use crate::sol::{SolDebugResult, SolView};

use super::SolItem;

impl SolItem {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        match self {
            Self::Autocharge(_) => Ok(()),
            Self::Booster(_) => Ok(()),
            Self::Character(_) => Ok(()),
            Self::Charge(_) => Ok(()),
            Self::Drone(_) => Ok(()),
            Self::Fighter(fighter) => fighter.debug_consistency_check(sol_view),
            Self::FwEffect(_) => Ok(()),
            Self::Implant(_) => Ok(()),
            Self::Module(module) => module.debug_consistency_check(sol_view),
            Self::ProjEffect(proj_effect) => proj_effect.debug_consistency_check(sol_view),
            Self::Rig(_) => Ok(()),
            Self::Ship(_) => Ok(()),
            Self::Skill(_) => Ok(()),
            Self::Stance(_) => Ok(()),
            Self::Subsystem(_) => Ok(()),
            Self::SwEffect(_) => Ok(()),
        }
    }
}
