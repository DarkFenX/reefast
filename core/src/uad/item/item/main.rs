use crate::{
    ad,
    def::{Count, FitKey, ItemId, ItemKey},
    err::basic::ItemKindMatchError,
    misc::{EffectMode, Spool},
    src::Src,
    uad::{
        Uad, UadAutocharge, UadBooster, UadCharacter, UadCharge, UadDrone, UadFighter, UadFwEffect, UadImplant,
        UadModule, UadProjEffect, UadRig, UadService, UadShip, UadSkill, UadStance, UadSubsystem, UadSwEffect,
        item::{Autocharges, ItemMutationData, Projs, UadEffectUpdates, UadProjRange},
    },
    util::{GetId, Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) enum UadItem {
    Autocharge(UadAutocharge),
    Booster(UadBooster),
    Character(UadCharacter),
    Charge(UadCharge),
    Drone(UadDrone),
    Fighter(UadFighter),
    FwEffect(UadFwEffect),
    Implant(UadImplant),
    Module(UadModule),
    ProjEffect(UadProjEffect),
    Service(UadService),
    Rig(UadRig),
    Ship(UadShip),
    Skill(UadSkill),
    Stance(UadStance),
    Subsystem(UadSubsystem),
    SwEffect(UadSwEffect),
}
impl UadItem {
    pub(crate) fn get_name(&self) -> &'static str {
        match self {
            Self::Autocharge(_) => UadAutocharge::get_name(),
            Self::Booster(_) => UadBooster::get_name(),
            Self::Character(_) => UadCharacter::get_name(),
            Self::Charge(_) => UadCharge::get_name(),
            Self::Drone(_) => UadDrone::get_name(),
            Self::Fighter(_) => UadFighter::get_name(),
            Self::FwEffect(_) => UadFwEffect::get_name(),
            Self::Implant(_) => UadImplant::get_name(),
            Self::Module(_) => UadModule::get_name(),
            Self::ProjEffect(_) => UadProjEffect::get_name(),
            Self::Rig(_) => UadRig::get_name(),
            Self::Service(_) => UadService::get_name(),
            Self::Ship(_) => UadShip::get_name(),
            Self::Skill(_) => UadSkill::get_name(),
            Self::Stance(_) => UadStance::get_name(),
            Self::Subsystem(_) => UadSubsystem::get_name(),
            Self::SwEffect(_) => UadSwEffect::get_name(),
        }
    }
    pub(crate) fn get_item_id(&self) -> ItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_item_id(),
            Self::Booster(booster) => booster.get_item_id(),
            Self::Character(character) => character.get_item_id(),
            Self::Charge(charge) => charge.get_item_id(),
            Self::Drone(drone) => drone.get_item_id(),
            Self::Fighter(fighter) => fighter.get_item_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_item_id(),
            Self::Implant(implant) => implant.get_item_id(),
            Self::Module(module) => module.get_item_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_item_id(),
            Self::Rig(rig) => rig.get_item_id(),
            Self::Service(service) => service.get_item_id(),
            Self::Ship(ship) => ship.get_item_id(),
            Self::Skill(skill) => skill.get_item_id(),
            Self::Stance(stance) => stance.get_item_id(),
            Self::Subsystem(subsystem) => subsystem.get_item_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_item_id(),
        }
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_item_id(),
            Self::Booster(booster) => booster.get_a_item_id(),
            Self::Character(character) => character.get_a_item_id(),
            Self::Charge(charge) => charge.get_a_item_id(),
            Self::Drone(drone) => drone.get_a_item_id(),
            Self::Fighter(fighter) => fighter.get_a_item_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_item_id(),
            Self::Implant(implant) => implant.get_a_item_id(),
            Self::Module(module) => module.get_a_item_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_item_id(),
            Self::Rig(rig) => rig.get_a_item_id(),
            Self::Service(service) => service.get_a_item_id(),
            Self::Ship(ship) => ship.get_a_item_id(),
            Self::Skill(skill) => skill.get_a_item_id(),
            Self::Stance(stance) => stance.get_a_item_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_item_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_item_id(),
        }
    }
    pub(crate) fn get_fit_key(&self) -> Option<FitKey> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_fit_key()),
            Self::Booster(booster) => Some(booster.get_fit_key()),
            Self::Character(character) => Some(character.get_fit_key()),
            Self::Charge(charge) => Some(charge.get_fit_key()),
            Self::Drone(drone) => Some(drone.get_fit_key()),
            Self::Fighter(fighter) => Some(fighter.get_fit_key()),
            Self::FwEffect(fw_effect) => Some(fw_effect.get_fit_key()),
            Self::Implant(implant) => Some(implant.get_fit_key()),
            Self::Module(module) => Some(module.get_fit_key()),
            Self::ProjEffect(_) => None,
            Self::Rig(rig) => Some(rig.get_fit_key()),
            Self::Service(service) => Some(service.get_fit_key()),
            Self::Ship(ship) => Some(ship.get_fit_key()),
            Self::Skill(skill) => Some(skill.get_fit_key()),
            Self::Stance(stance) => Some(stance.get_fit_key()),
            Self::Subsystem(subsystem) => Some(subsystem.get_fit_key()),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_reffs(),
            Self::Booster(booster) => booster.get_reffs(),
            Self::Character(character) => character.get_reffs(),
            Self::Charge(charge) => charge.get_reffs(),
            Self::Drone(drone) => drone.get_reffs(),
            Self::Fighter(fighter) => fighter.get_reffs(),
            Self::FwEffect(fw_effect) => fw_effect.get_reffs(),
            Self::Implant(implant) => implant.get_reffs(),
            Self::Module(module) => module.get_reffs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_reffs(),
            Self::Rig(rig) => rig.get_reffs(),
            Self::Service(service) => service.get_reffs(),
            Self::Ship(ship) => ship.get_reffs(),
            Self::Skill(skill) => skill.get_reffs(),
            Self::Stance(stance) => stance.get_reffs(),
            Self::Subsystem(subsystem) => subsystem.get_reffs(),
            Self::SwEffect(sw_effect) => sw_effect.get_reffs(),
        }
    }
    pub(crate) fn start_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.start_all_reffs(reuse_eupdates, src),
            Self::Booster(booster) => booster.start_all_reffs(reuse_eupdates, src),
            Self::Character(character) => character.start_all_reffs(reuse_eupdates, src),
            Self::Charge(charge) => charge.start_all_reffs(reuse_eupdates, src),
            Self::Drone(drone) => drone.start_all_reffs(reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.start_all_reffs(reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.start_all_reffs(reuse_eupdates, src),
            Self::Implant(implant) => implant.start_all_reffs(reuse_eupdates, src),
            Self::Module(module) => module.start_all_reffs(reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.start_all_reffs(reuse_eupdates, src),
            Self::Rig(rig) => rig.start_all_reffs(reuse_eupdates, src),
            Self::Service(service) => service.start_all_reffs(reuse_eupdates, src),
            Self::Ship(ship) => ship.start_all_reffs(reuse_eupdates, src),
            Self::Skill(skill) => skill.start_all_reffs(reuse_eupdates, src),
            Self::Stance(stance) => stance.start_all_reffs(reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.start_all_reffs(reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.start_all_reffs(reuse_eupdates, src),
        }
    }
    pub(crate) fn stop_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.stop_all_reffs(reuse_eupdates, src),
            Self::Booster(booster) => booster.stop_all_reffs(reuse_eupdates, src),
            Self::Character(character) => character.stop_all_reffs(reuse_eupdates, src),
            Self::Charge(charge) => charge.stop_all_reffs(reuse_eupdates, src),
            Self::Drone(drone) => drone.stop_all_reffs(reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.stop_all_reffs(reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.stop_all_reffs(reuse_eupdates, src),
            Self::Implant(implant) => implant.stop_all_reffs(reuse_eupdates, src),
            Self::Module(module) => module.stop_all_reffs(reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.stop_all_reffs(reuse_eupdates, src),
            Self::Rig(rig) => rig.stop_all_reffs(reuse_eupdates, src),
            Self::Service(service) => service.stop_all_reffs(reuse_eupdates, src),
            Self::Ship(ship) => ship.stop_all_reffs(reuse_eupdates, src),
            Self::Skill(skill) => skill.stop_all_reffs(reuse_eupdates, src),
            Self::Stance(stance) => stance.stop_all_reffs(reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.stop_all_reffs(reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.stop_all_reffs(reuse_eupdates, src),
        }
    }
    pub(crate) fn get_effect_mode(&self, a_effect_id: &ad::AEffectId) -> EffectMode {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_effect_mode(a_effect_id),
            Self::Booster(booster) => booster.get_effect_mode(a_effect_id),
            Self::Character(character) => character.get_effect_mode(a_effect_id),
            Self::Charge(charge) => charge.get_effect_mode(a_effect_id),
            Self::Drone(drone) => drone.get_effect_mode(a_effect_id),
            Self::Fighter(fighter) => fighter.get_effect_mode(a_effect_id),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_mode(a_effect_id),
            Self::Implant(implant) => implant.get_effect_mode(a_effect_id),
            Self::Module(module) => module.get_effect_mode(a_effect_id),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_mode(a_effect_id),
            Self::Rig(rig) => rig.get_effect_mode(a_effect_id),
            Self::Service(service) => service.get_effect_mode(a_effect_id),
            Self::Ship(ship) => ship.get_effect_mode(a_effect_id),
            Self::Skill(skill) => skill.get_effect_mode(a_effect_id),
            Self::Stance(stance) => stance.get_effect_mode(a_effect_id),
            Self::Subsystem(subsystem) => subsystem.get_effect_mode(a_effect_id),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_mode(a_effect_id),
        }
    }
    pub(crate) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Booster(booster) => booster.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Character(character) => character.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Charge(charge) => charge.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Drone(drone) => drone.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Implant(implant) => implant.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Module(module) => module.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Rig(rig) => rig.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Service(service) => service.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Ship(ship) => ship.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Skill(skill) => skill.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Stance(stance) => stance.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src),
        }
    }
    pub(crate) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_modes(modes, reuse_eupdates, src),
            Self::Booster(booster) => booster.set_effect_modes(modes, reuse_eupdates, src),
            Self::Character(character) => character.set_effect_modes(modes, reuse_eupdates, src),
            Self::Charge(charge) => charge.set_effect_modes(modes, reuse_eupdates, src),
            Self::Drone(drone) => drone.set_effect_modes(modes, reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.set_effect_modes(modes, reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_modes(modes, reuse_eupdates, src),
            Self::Implant(implant) => implant.set_effect_modes(modes, reuse_eupdates, src),
            Self::Module(module) => module.set_effect_modes(modes, reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_modes(modes, reuse_eupdates, src),
            Self::Rig(rig) => rig.set_effect_modes(modes, reuse_eupdates, src),
            Self::Service(service) => service.set_effect_modes(modes, reuse_eupdates, src),
            Self::Ship(ship) => ship.set_effect_modes(modes, reuse_eupdates, src),
            Self::Skill(skill) => skill.set_effect_modes(modes, reuse_eupdates, src),
            Self::Stance(stance) => stance.set_effect_modes(modes, reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.set_effect_modes(modes, reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_modes(modes, reuse_eupdates, src),
        }
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_state(),
            Self::Booster(booster) => booster.get_a_state(),
            Self::Character(character) => character.get_a_state(),
            Self::Charge(charge) => charge.get_a_state(),
            Self::Drone(drone) => drone.get_a_state(),
            Self::Fighter(fighter) => fighter.get_a_state(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_state(),
            Self::Implant(implant) => implant.get_a_state(),
            Self::Module(module) => module.get_a_state(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_state(),
            Self::Rig(rig) => rig.get_a_state(),
            Self::Service(service) => service.get_a_state(),
            Self::Ship(ship) => ship.get_a_state(),
            Self::Skill(skill) => skill.get_a_state(),
            Self::Stance(stance) => stance.get_a_state(),
            Self::Subsystem(subsystem) => subsystem.get_a_state(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_state(),
        }
    }
    pub(crate) fn update_a_data(&mut self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.update_a_data(reuse_eupdates, src),
            Self::Booster(booster) => booster.update_a_data(reuse_eupdates, src),
            Self::Character(character) => character.update_a_data(reuse_eupdates, src),
            Self::Charge(charge) => charge.update_a_data(reuse_eupdates, src),
            Self::Drone(drone) => drone.update_a_data(reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.update_a_data(reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.update_a_data(reuse_eupdates, src),
            Self::Implant(implant) => implant.update_a_data(reuse_eupdates, src),
            Self::Module(module) => module.update_a_data(reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.update_a_data(reuse_eupdates, src),
            Self::Rig(rig) => rig.update_a_data(reuse_eupdates, src),
            Self::Service(service) => service.update_a_data(reuse_eupdates, src),
            Self::Ship(ship) => ship.update_a_data(reuse_eupdates, src),
            Self::Skill(skill) => skill.update_a_data(reuse_eupdates, src),
            Self::Stance(stance) => stance.update_a_data(reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.update_a_data(reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.update_a_data(reuse_eupdates, src),
        }
    }
    pub(crate) fn is_loaded(&self) -> bool {
        match self {
            Self::Autocharge(autocharge) => autocharge.is_loaded(),
            Self::Booster(booster) => booster.is_loaded(),
            Self::Character(character) => character.is_loaded(),
            Self::Charge(charge) => charge.is_loaded(),
            Self::Drone(drone) => drone.is_loaded(),
            Self::Fighter(fighter) => fighter.is_loaded(),
            Self::FwEffect(fw_effect) => fw_effect.is_loaded(),
            Self::Implant(implant) => implant.is_loaded(),
            Self::Module(module) => module.is_loaded(),
            Self::ProjEffect(proj_effect) => proj_effect.is_loaded(),
            Self::Rig(rig) => rig.is_loaded(),
            Self::Service(service) => service.is_loaded(),
            Self::Ship(ship) => ship.is_loaded(),
            Self::Skill(skill) => skill.is_loaded(),
            Self::Stance(stance) => stance.is_loaded(),
            Self::Subsystem(subsystem) => subsystem.is_loaded(),
            Self::SwEffect(sw_effect) => sw_effect.is_loaded(),
        }
    }
    pub(crate) fn can_receive_projs(&self) -> bool {
        matches!(self, Self::Drone(_) | Self::Fighter(_) | Self::Ship(_))
    }
    pub(crate) fn get_projs(&self) -> Option<&Projs> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs()),
            Self::Charge(charge) => Some(charge.get_projs()),
            Self::Drone(drone) => Some(drone.get_projs()),
            Self::Fighter(fighter) => Some(fighter.get_projs()),
            Self::Module(module) => Some(module.get_projs()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs()),
            _ => None,
        }
    }
    pub(crate) fn get_projs_mut(&mut self) -> Option<&mut Projs> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs_mut()),
            Self::Charge(charge) => Some(charge.get_projs_mut()),
            Self::Drone(drone) => Some(drone.get_projs_mut()),
            Self::Fighter(fighter) => Some(fighter.get_projs_mut()),
            Self::Module(module) => Some(module.get_projs_mut()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs_mut()),
            _ => None,
        }
    }
    pub(crate) fn iter_projs(&self) -> Option<impl ExactSizeIterator<Item = (ItemKey, Option<UadProjRange>)>> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter()),
            Self::Charge(charge) => Some(charge.get_projs().iter()),
            Self::Drone(drone) => Some(drone.get_projs().iter()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter()),
            Self::Module(module) => Some(module.get_projs().iter()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter()),
            _ => None,
        }
    }
    pub(crate) fn iter_projectees(&self) -> Option<impl ExactSizeIterator<Item = ItemKey>> {
        match self {
            Self::Autocharge(autocharge) => Some(autocharge.get_projs().iter_projectees()),
            Self::Charge(charge) => Some(charge.get_projs().iter_projectees()),
            Self::Drone(drone) => Some(drone.get_projs().iter_projectees()),
            Self::Fighter(fighter) => Some(fighter.get_projs().iter_projectees()),
            Self::Module(module) => Some(module.get_projs().iter_projectees()),
            Self::ProjEffect(proj_effect) => Some(proj_effect.get_projs().iter_projectees()),
            _ => None,
        }
    }
    pub(crate) fn get_charge_key(&self) -> Option<ItemKey> {
        match self {
            Self::Module(module) => module.get_charge_key(),
            _ => None,
        }
    }
    pub(crate) fn get_charge_count(&self, uad: &Uad) -> Option<Count> {
        match self {
            Self::Module(module) => module.get_charge_count(uad),
            _ => None,
        }
    }
    pub(crate) fn get_spool(&self) -> Option<Spool> {
        match self {
            Self::Module(module) => module.get_spool(),
            _ => None,
        }
    }

    pub(crate) fn get_autocharges(&self) -> Option<&Autocharges> {
        match self {
            Self::Fighter(fighter) => Some(fighter.get_autocharges()),
            _ => None,
        }
    }
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        match self {
            Self::Drone(drone) => drone.get_mutation_data(),
            Self::Module(module) => module.get_mutation_data(),
            _ => None,
        }
    }
    // Extractors of specific items
    pub(crate) fn get_autocharge(&self) -> Result<&UadAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadAutocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_autocharge_mut(&mut self) -> Result<&mut UadAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadAutocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_booster(&self) -> Result<&UadBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadBooster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_booster_mut(&mut self) -> Result<&mut UadBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadBooster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_character(&self) -> Result<&UadCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadCharacter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_character_mut(&mut self) -> Result<&mut UadCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadCharacter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_charge(&self) -> Result<&UadCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadCharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_charge_mut(&mut self) -> Result<&mut UadCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadCharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_drone(&self) -> Result<&UadDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadDrone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_drone_mut(&mut self) -> Result<&mut UadDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadDrone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fighter(&self) -> Result<&UadFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadFighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fighter_mut(&mut self) -> Result<&mut UadFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadFighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fw_effect(&self) -> Result<&UadFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadFwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fw_effect_mut(&mut self) -> Result<&mut UadFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadFwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_implant(&self) -> Result<&UadImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadImplant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_implant_mut(&mut self) -> Result<&mut UadImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadImplant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_module(&self) -> Result<&UadModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadModule::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_module_mut(&mut self) -> Result<&mut UadModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadModule::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_proj_effect(&self) -> Result<&UadProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_proj_effect_mut(&mut self) -> Result<&mut UadProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_rig(&self) -> Result<&UadRig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadRig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_rig_mut(&mut self) -> Result<&mut UadRig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadRig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_service(&self) -> Result<&UadService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadService::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_service_mut(&mut self) -> Result<&mut UadService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadService::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_ship(&self) -> Result<&UadShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadShip::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_ship_mut(&mut self) -> Result<&mut UadShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadShip::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_skill(&self) -> Result<&UadSkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadSkill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_skill_mut(&mut self) -> Result<&mut UadSkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadSkill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_stance(&self) -> Result<&UadStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadStance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_stance_mut(&mut self) -> Result<&mut UadStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadStance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_subsystem(&self) -> Result<&UadSubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadSubsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_subsystem_mut(&mut self) -> Result<&mut UadSubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadSubsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_sw_effect(&self) -> Result<&UadSwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadSwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_sw_effect_mut(&mut self) -> Result<&mut UadSwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UadSwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    // Service-specific getters
    // TODO: consider moving to service specific item extensions
    pub(crate) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_group_id(),
            Self::Booster(booster) => booster.get_a_group_id(),
            Self::Character(character) => character.get_a_group_id(),
            Self::Charge(charge) => charge.get_a_group_id(),
            Self::Drone(drone) => drone.get_a_group_id(),
            Self::Fighter(fighter) => fighter.get_a_group_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_group_id(),
            Self::Implant(implant) => implant.get_a_group_id(),
            Self::Module(module) => module.get_a_group_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_group_id(),
            Self::Rig(rig) => rig.get_a_group_id(),
            Self::Service(service) => service.get_a_group_id(),
            Self::Ship(ship) => ship.get_a_group_id(),
            Self::Skill(skill) => skill.get_a_group_id(),
            Self::Stance(stance) => stance.get_a_group_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_group_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_group_id(),
        }
    }
    pub(crate) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_category_id(),
            Self::Booster(booster) => booster.get_a_category_id(),
            Self::Character(character) => character.get_a_category_id(),
            Self::Charge(charge) => charge.get_a_category_id(),
            Self::Drone(drone) => drone.get_a_category_id(),
            Self::Fighter(fighter) => fighter.get_a_category_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_category_id(),
            Self::Implant(implant) => implant.get_a_category_id(),
            Self::Module(module) => module.get_a_category_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_category_id(),
            Self::Rig(rig) => rig.get_a_category_id(),
            Self::Service(service) => service.get_a_category_id(),
            Self::Ship(ship) => ship.get_a_category_id(),
            Self::Skill(skill) => skill.get_a_category_id(),
            Self::Stance(stance) => stance.get_a_category_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_category_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_category_id(),
        }
    }
    pub(crate) fn get_a_attr(&self, a_attr_id: &ad::AAttrId) -> Option<ad::AAttrVal> {
        match self.get_a_attrs() {
            Some(attrs) => attrs.get(a_attr_id).copied(),
            None => None,
        }
    }
    pub(crate) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_attrs(),
            Self::Booster(booster) => booster.get_a_attrs(),
            Self::Character(character) => character.get_a_attrs(),
            Self::Charge(charge) => charge.get_a_attrs(),
            Self::Drone(drone) => drone.get_a_attrs(),
            Self::Fighter(fighter) => fighter.get_a_attrs(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_attrs(),
            Self::Implant(implant) => implant.get_a_attrs(),
            Self::Module(module) => module.get_a_attrs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_attrs(),
            Self::Rig(rig) => rig.get_a_attrs(),
            Self::Service(service) => service.get_a_attrs(),
            Self::Ship(ship) => ship.get_a_attrs(),
            Self::Skill(skill) => skill.get_a_attrs(),
            Self::Stance(stance) => stance.get_a_attrs(),
            Self::Subsystem(subsystem) => subsystem.get_a_attrs(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_attrs(),
        }
    }
    pub(crate) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_effect_datas(),
            Self::Booster(booster) => booster.get_a_effect_datas(),
            Self::Character(character) => character.get_a_effect_datas(),
            Self::Charge(charge) => charge.get_a_effect_datas(),
            Self::Drone(drone) => drone.get_a_effect_datas(),
            Self::Fighter(fighter) => fighter.get_a_effect_datas(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_effect_datas(),
            Self::Implant(implant) => implant.get_a_effect_datas(),
            Self::Module(module) => module.get_a_effect_datas(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_effect_datas(),
            Self::Rig(rig) => rig.get_a_effect_datas(),
            Self::Service(service) => service.get_a_effect_datas(),
            Self::Ship(ship) => ship.get_a_effect_datas(),
            Self::Skill(skill) => skill.get_a_effect_datas(),
            Self::Stance(stance) => stance.get_a_effect_datas(),
            Self::Subsystem(subsystem) => subsystem.get_a_effect_datas(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_effect_datas(),
        }
    }
    pub(crate) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_defeff_id(),
            Self::Booster(booster) => booster.get_a_defeff_id(),
            Self::Character(character) => character.get_a_defeff_id(),
            Self::Charge(charge) => charge.get_a_defeff_id(),
            Self::Drone(drone) => drone.get_a_defeff_id(),
            Self::Fighter(fighter) => fighter.get_a_defeff_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_defeff_id(),
            Self::Implant(implant) => implant.get_a_defeff_id(),
            Self::Module(module) => module.get_a_defeff_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_defeff_id(),
            Self::Rig(rig) => rig.get_a_defeff_id(),
            Self::Service(service) => service.get_a_defeff_id(),
            Self::Ship(ship) => ship.get_a_defeff_id(),
            Self::Skill(skill) => skill.get_a_defeff_id(),
            Self::Stance(stance) => stance.get_a_defeff_id(),
            Self::Subsystem(subsystem) => subsystem.get_a_defeff_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_defeff_id(),
        }
    }
    pub(crate) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_skill_reqs(),
            Self::Booster(booster) => booster.get_a_skill_reqs(),
            Self::Character(character) => character.get_a_skill_reqs(),
            Self::Charge(charge) => charge.get_a_skill_reqs(),
            Self::Drone(drone) => drone.get_a_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_a_skill_reqs(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_skill_reqs(),
            Self::Implant(implant) => implant.get_a_skill_reqs(),
            Self::Module(module) => module.get_a_skill_reqs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_skill_reqs(),
            Self::Rig(rig) => rig.get_a_skill_reqs(),
            Self::Service(service) => service.get_a_skill_reqs(),
            Self::Ship(ship) => ship.get_a_skill_reqs(),
            Self::Skill(skill) => skill.get_a_skill_reqs(),
            Self::Stance(stance) => stance.get_a_skill_reqs(),
            Self::Subsystem(subsystem) => subsystem.get_a_skill_reqs(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_skill_reqs(),
        }
    }
    pub(crate) fn get_effective_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(booster) => booster.get_a_skill_reqs(),
            Self::Character(_) => None,
            Self::Charge(charge) => charge.get_a_skill_reqs(),
            Self::Drone(drone) => drone.get_a_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_a_skill_reqs(),
            Self::FwEffect(_) => None,
            Self::Implant(implant) => implant.get_a_skill_reqs(),
            Self::Module(module) => module.get_a_skill_reqs(),
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Service(_) => None,
            Self::Ship(ship) => ship.get_a_skill_reqs(),
            Self::Skill(skill) => skill.get_a_skill_reqs(),
            Self::Stance(_) => None,
            Self::Subsystem(subsystem) => subsystem.get_a_skill_reqs(),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_a_xt(&self) -> Option<&ad::AItemXt> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_a_xt(),
            Self::Booster(booster) => booster.get_a_xt(),
            Self::Character(character) => character.get_a_xt(),
            Self::Charge(charge) => charge.get_a_xt(),
            Self::Drone(drone) => drone.get_a_xt(),
            Self::Fighter(fighter) => fighter.get_a_xt(),
            Self::FwEffect(fw_effect) => fw_effect.get_a_xt(),
            Self::Implant(implant) => implant.get_a_xt(),
            Self::Module(module) => module.get_a_xt(),
            Self::ProjEffect(proj_effect) => proj_effect.get_a_xt(),
            Self::Rig(rig) => rig.get_a_xt(),
            Self::Service(service) => service.get_a_xt(),
            Self::Ship(ship) => ship.get_a_xt(),
            Self::Skill(skill) => skill.get_a_xt(),
            Self::Stance(stance) => stance.get_a_xt(),
            Self::Subsystem(subsystem) => subsystem.get_a_xt(),
            Self::SwEffect(sw_effect) => sw_effect.get_a_xt(),
        }
    }
}
impl Named for UadItem {
    fn get_name() -> &'static str {
        "Item"
    }
}
impl GetId<ItemId> for UadItem {
    fn get_id(&self) -> ItemId {
        self.get_item_id()
    }
}
