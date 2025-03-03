use crate::{
    defs::{SolFitId, SolFleetId, SolItemId},
    sol::{SolDmgProfile, uad::fit::SolFit},
};

pub struct SolFitInfo {
    pub id: SolFitId,
    pub fleet: Option<SolFleetId>,
    pub character: Option<SolItemId>,
    pub skills: Vec<SolItemId>,
    pub implants: Vec<SolItemId>,
    pub boosters: Vec<SolItemId>,
    pub ship: Option<SolItemId>,
    pub stance: Option<SolItemId>,
    pub subsystems: Vec<SolItemId>,
    pub mods_high: Vec<Option<SolItemId>>,
    pub mods_mid: Vec<Option<SolItemId>>,
    pub mods_low: Vec<Option<SolItemId>>,
    pub rigs: Vec<SolItemId>,
    pub drones: Vec<SolItemId>,
    pub fighters: Vec<SolItemId>,
    pub fw_effects: Vec<SolItemId>,
    pub rah_incoming_dmg: Option<SolDmgProfile>,
}
impl SolFitInfo {
    pub(in crate::sol) fn new(
        id: SolFitId,
        fleet: Option<SolFleetId>,
        character: Option<SolItemId>,
        skills: Vec<SolItemId>,
        implants: Vec<SolItemId>,
        boosters: Vec<SolItemId>,
        ship: Option<SolItemId>,
        stance: Option<SolItemId>,
        subsystems: Vec<SolItemId>,
        mods_high: Vec<Option<SolItemId>>,
        mods_mid: Vec<Option<SolItemId>>,
        mods_low: Vec<Option<SolItemId>>,
        rigs: Vec<SolItemId>,
        drones: Vec<SolItemId>,
        fighters: Vec<SolItemId>,
        fw_effects: Vec<SolItemId>,
        rah_incoming_dmg: Option<SolDmgProfile>,
    ) -> Self {
        Self {
            id,
            fleet,
            character,
            skills,
            implants,
            boosters,
            ship,
            stance,
            subsystems,
            mods_high,
            mods_mid,
            mods_low,
            rigs,
            drones,
            fighters,
            fw_effects,
            rah_incoming_dmg,
        }
    }
}
impl From<&SolFit> for SolFitInfo {
    fn from(fit: &SolFit) -> Self {
        Self::new(
            fit.id,
            fit.fleet,
            fit.character,
            fit.skills.values().map(|v| v.item_id).collect(),
            fit.implants.iter().copied().collect(),
            fit.boosters.iter().copied().collect(),
            fit.ship,
            fit.stance,
            fit.subsystems.iter().copied().collect(),
            fit.mods_high.inner().clone(),
            fit.mods_mid.inner().clone(),
            fit.mods_low.inner().clone(),
            fit.rigs.iter().copied().collect(),
            fit.drones.iter().copied().collect(),
            fit.fighters.iter().copied().collect(),
            fit.fw_effects.iter().copied().collect(),
            fit.rah_incoming_dmg,
        )
    }
}
