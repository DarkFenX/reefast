use crate::{
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState},
    def::{Count, ItemId},
    err::basic::ItemKindMatchError,
    misc::{EffectMode, Spool},
    rd::{REffectKey, RItemAXt},
    src::Src,
    ud::{
        UAutocharge, UBooster, UCharacter, UCharge, UData, UDrone, UFighter, UFitKey, UFwEffect, UImplant, UItemKey,
        UModule, UProjEffect, URig, UService, UShip, USkill, UStance, USubsystem, USwEffect,
        item::{Autocharges, ItemMutationData, Projs, UEffectUpdates, UProjRange},
    },
    util::{GetId, Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) enum UItem {
    Autocharge(UAutocharge),
    Booster(UBooster),
    Character(UCharacter),
    Charge(UCharge),
    Drone(UDrone),
    Fighter(UFighter),
    FwEffect(UFwEffect),
    Implant(UImplant),
    Module(UModule),
    ProjEffect(UProjEffect),
    Service(UService),
    Rig(URig),
    Ship(UShip),
    Skill(USkill),
    Stance(UStance),
    Subsystem(USubsystem),
    SwEffect(USwEffect),
}
impl UItem {
    pub(crate) fn get_name(&self) -> &'static str {
        match self {
            Self::Autocharge(_) => UAutocharge::get_name(),
            Self::Booster(_) => UBooster::get_name(),
            Self::Character(_) => UCharacter::get_name(),
            Self::Charge(_) => UCharge::get_name(),
            Self::Drone(_) => UDrone::get_name(),
            Self::Fighter(_) => UFighter::get_name(),
            Self::FwEffect(_) => UFwEffect::get_name(),
            Self::Implant(_) => UImplant::get_name(),
            Self::Module(_) => UModule::get_name(),
            Self::ProjEffect(_) => UProjEffect::get_name(),
            Self::Rig(_) => URig::get_name(),
            Self::Service(_) => UService::get_name(),
            Self::Ship(_) => UShip::get_name(),
            Self::Skill(_) => USkill::get_name(),
            Self::Stance(_) => UStance::get_name(),
            Self::Subsystem(_) => USubsystem::get_name(),
            Self::SwEffect(_) => USwEffect::get_name(),
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
    pub(crate) fn get_type_id(&self) -> AItemId {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_type_id(),
            Self::Booster(booster) => booster.get_type_id(),
            Self::Character(character) => character.get_type_id(),
            Self::Charge(charge) => charge.get_type_id(),
            Self::Drone(drone) => drone.get_type_id(),
            Self::Fighter(fighter) => fighter.get_type_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_type_id(),
            Self::Implant(implant) => implant.get_type_id(),
            Self::Module(module) => module.get_type_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_type_id(),
            Self::Rig(rig) => rig.get_type_id(),
            Self::Service(service) => service.get_type_id(),
            Self::Ship(ship) => ship.get_type_id(),
            Self::Skill(skill) => skill.get_type_id(),
            Self::Stance(stance) => stance.get_type_id(),
            Self::Subsystem(subsystem) => subsystem.get_type_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_type_id(),
        }
    }
    pub(crate) fn get_fit_key(&self) -> Option<UFitKey> {
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
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
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
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.update_reffs(reuse_eupdates, src),
            Self::Booster(booster) => booster.update_reffs(reuse_eupdates, src),
            Self::Character(character) => character.update_reffs(reuse_eupdates, src),
            Self::Charge(charge) => charge.update_reffs(reuse_eupdates, src),
            Self::Drone(drone) => drone.update_reffs(reuse_eupdates, src),
            Self::Fighter(fighter) => fighter.update_reffs(reuse_eupdates, src),
            Self::FwEffect(fw_effect) => fw_effect.update_reffs(reuse_eupdates, src),
            Self::Implant(implant) => implant.update_reffs(reuse_eupdates, src),
            Self::Module(module) => module.update_reffs(reuse_eupdates, src),
            Self::ProjEffect(proj_effect) => proj_effect.update_reffs(reuse_eupdates, src),
            Self::Rig(rig) => rig.update_reffs(reuse_eupdates, src),
            Self::Service(service) => service.update_reffs(reuse_eupdates, src),
            Self::Ship(ship) => ship.update_reffs(reuse_eupdates, src),
            Self::Skill(skill) => skill.update_reffs(reuse_eupdates, src),
            Self::Stance(stance) => stance.update_reffs(reuse_eupdates, src),
            Self::Subsystem(subsystem) => subsystem.update_reffs(reuse_eupdates, src),
            Self::SwEffect(sw_effect) => sw_effect.update_reffs(reuse_eupdates, src),
        }
    }
    pub(crate) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
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
    pub(crate) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_effect_key_mode(effect_key),
            Self::Booster(booster) => booster.get_effect_key_mode(effect_key),
            Self::Character(character) => character.get_effect_key_mode(effect_key),
            Self::Charge(charge) => charge.get_effect_key_mode(effect_key),
            Self::Drone(drone) => drone.get_effect_key_mode(effect_key),
            Self::Fighter(fighter) => fighter.get_effect_key_mode(effect_key),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_key_mode(effect_key),
            Self::Implant(implant) => implant.get_effect_key_mode(effect_key),
            Self::Module(module) => module.get_effect_key_mode(effect_key),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_key_mode(effect_key),
            Self::Rig(rig) => rig.get_effect_key_mode(effect_key),
            Self::Service(service) => service.get_effect_key_mode(effect_key),
            Self::Ship(ship) => ship.get_effect_key_mode(effect_key),
            Self::Skill(skill) => skill.get_effect_key_mode(effect_key),
            Self::Stance(stance) => stance.get_effect_key_mode(effect_key),
            Self::Subsystem(subsystem) => subsystem.get_effect_key_mode(effect_key),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_key_mode(effect_key),
        }
    }
    pub(crate) fn set_effect_mode(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_mode(effect_id, effect_mode, src),
            Self::Booster(booster) => booster.set_effect_mode(effect_id, effect_mode, src),
            Self::Character(character) => character.set_effect_mode(effect_id, effect_mode, src),
            Self::Charge(charge) => charge.set_effect_mode(effect_id, effect_mode, src),
            Self::Drone(drone) => drone.set_effect_mode(effect_id, effect_mode, src),
            Self::Fighter(fighter) => fighter.set_effect_mode(effect_id, effect_mode, src),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_mode(effect_id, effect_mode, src),
            Self::Implant(implant) => implant.set_effect_mode(effect_id, effect_mode, src),
            Self::Module(module) => module.set_effect_mode(effect_id, effect_mode, src),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_mode(effect_id, effect_mode, src),
            Self::Rig(rig) => rig.set_effect_mode(effect_id, effect_mode, src),
            Self::Service(service) => service.set_effect_mode(effect_id, effect_mode, src),
            Self::Ship(ship) => ship.set_effect_mode(effect_id, effect_mode, src),
            Self::Skill(skill) => skill.set_effect_mode(effect_id, effect_mode, src),
            Self::Stance(stance) => stance.set_effect_mode(effect_id, effect_mode, src),
            Self::Subsystem(subsystem) => subsystem.set_effect_mode(effect_id, effect_mode, src),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_mode(effect_id, effect_mode, src),
        }
    }
    pub(crate) fn set_effect_modes(&mut self, effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.set_effect_modes(effect_modes, src),
            Self::Booster(booster) => booster.set_effect_modes(effect_modes, src),
            Self::Character(character) => character.set_effect_modes(effect_modes, src),
            Self::Charge(charge) => charge.set_effect_modes(effect_modes, src),
            Self::Drone(drone) => drone.set_effect_modes(effect_modes, src),
            Self::Fighter(fighter) => fighter.set_effect_modes(effect_modes, src),
            Self::FwEffect(fw_effect) => fw_effect.set_effect_modes(effect_modes, src),
            Self::Implant(implant) => implant.set_effect_modes(effect_modes, src),
            Self::Module(module) => module.set_effect_modes(effect_modes, src),
            Self::ProjEffect(proj_effect) => proj_effect.set_effect_modes(effect_modes, src),
            Self::Rig(rig) => rig.set_effect_modes(effect_modes, src),
            Self::Service(service) => service.set_effect_modes(effect_modes, src),
            Self::Ship(ship) => ship.set_effect_modes(effect_modes, src),
            Self::Skill(skill) => skill.set_effect_modes(effect_modes, src),
            Self::Stance(stance) => stance.set_effect_modes(effect_modes, src),
            Self::Subsystem(subsystem) => subsystem.set_effect_modes(effect_modes, src),
            Self::SwEffect(sw_effect) => sw_effect.set_effect_modes(effect_modes, src),
        }
    }
    pub(crate) fn get_state(&self) -> AState {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_state(),
            Self::Booster(booster) => booster.get_state(),
            Self::Character(character) => character.get_state(),
            Self::Charge(charge) => charge.get_state(),
            Self::Drone(drone) => drone.get_state(),
            Self::Fighter(fighter) => fighter.get_state(),
            Self::FwEffect(fw_effect) => fw_effect.get_state(),
            Self::Implant(implant) => implant.get_state(),
            Self::Module(module) => module.get_state(),
            Self::ProjEffect(proj_effect) => proj_effect.get_state(),
            Self::Rig(rig) => rig.get_state(),
            Self::Service(service) => service.get_state(),
            Self::Ship(ship) => ship.get_state(),
            Self::Skill(skill) => skill.get_state(),
            Self::Stance(stance) => stance.get_state(),
            Self::Subsystem(subsystem) => subsystem.get_state(),
            Self::SwEffect(sw_effect) => sw_effect.get_state(),
        }
    }
    pub(crate) fn src_changed(&mut self, src: &Src) {
        match self {
            Self::Autocharge(autocharge) => autocharge.src_changed(src),
            Self::Booster(booster) => booster.src_changed(src),
            Self::Character(character) => character.src_changed(src),
            Self::Charge(charge) => charge.src_changed(src),
            Self::Drone(drone) => drone.src_changed(src),
            Self::Fighter(fighter) => fighter.src_changed(src),
            Self::FwEffect(fw_effect) => fw_effect.src_changed(src),
            Self::Implant(implant) => implant.src_changed(src),
            Self::Module(module) => module.src_changed(src),
            Self::ProjEffect(proj_effect) => proj_effect.src_changed(src),
            Self::Rig(rig) => rig.src_changed(src),
            Self::Service(service) => service.src_changed(src),
            Self::Ship(ship) => ship.src_changed(src),
            Self::Skill(skill) => skill.src_changed(src),
            Self::Stance(stance) => stance.src_changed(src),
            Self::Subsystem(subsystem) => subsystem.update_a_data(src),
            Self::SwEffect(sw_effect) => sw_effect.src_changed(src),
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
    pub(crate) fn iter_projs(&self) -> Option<impl ExactSizeIterator<Item = (UItemKey, Option<UProjRange>)>> {
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
    pub(crate) fn iter_projectees(&self) -> Option<impl ExactSizeIterator<Item = UItemKey>> {
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
    pub(crate) fn get_charge_key(&self) -> Option<UItemKey> {
        match self {
            Self::Module(module) => module.get_charge_key(),
            _ => None,
        }
    }
    pub(crate) fn get_charge_count(&self, u_data: &UData) -> Option<Count> {
        match self {
            Self::Module(module) => module.get_charge_count(u_data),
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
    pub(crate) fn get_autocharges_mut(&mut self) -> Option<&mut Autocharges> {
        match self {
            Self::Fighter(fighter) => Some(fighter.get_autocharges_mut()),
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
    pub(crate) fn get_autocharge(&self) -> Result<&UAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UAutocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_autocharge_mut(&mut self) -> Result<&mut UAutocharge, ItemKindMatchError> {
        match self {
            Self::Autocharge(autocharge) => Ok(autocharge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UAutocharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_booster(&self) -> Result<&UBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UBooster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_booster_mut(&mut self) -> Result<&mut UBooster, ItemKindMatchError> {
        match self {
            Self::Booster(booster) => Ok(booster),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UBooster::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_character(&self) -> Result<&UCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharacter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_character_mut(&mut self) -> Result<&mut UCharacter, ItemKindMatchError> {
        match self {
            Self::Character(character) => Ok(character),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharacter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_charge(&self) -> Result<&UCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_charge_mut(&mut self) -> Result<&mut UCharge, ItemKindMatchError> {
        match self {
            Self::Charge(charge) => Ok(charge),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UCharge::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_drone(&self) -> Result<&UDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UDrone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_drone_mut(&mut self) -> Result<&mut UDrone, ItemKindMatchError> {
        match self {
            Self::Drone(drone) => Ok(drone),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UDrone::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fighter(&self) -> Result<&UFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fighter_mut(&mut self) -> Result<&mut UFighter, ItemKindMatchError> {
        match self {
            Self::Fighter(fighter) => Ok(fighter),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFighter::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fw_effect(&self) -> Result<&UFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_fw_effect_mut(&mut self) -> Result<&mut UFwEffect, ItemKindMatchError> {
        match self {
            Self::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UFwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_implant(&self) -> Result<&UImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UImplant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_implant_mut(&mut self) -> Result<&mut UImplant, ItemKindMatchError> {
        match self {
            Self::Implant(implant) => Ok(implant),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UImplant::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_module(&self) -> Result<&UModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UModule::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_module_mut(&mut self) -> Result<&mut UModule, ItemKindMatchError> {
        match self {
            Self::Module(module) => Ok(module),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UModule::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_proj_effect(&self) -> Result<&UProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_proj_effect_mut(&mut self) -> Result<&mut UProjEffect, ItemKindMatchError> {
        match self {
            Self::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UProjEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_rig(&self) -> Result<&URig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: URig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_rig_mut(&mut self) -> Result<&mut URig, ItemKindMatchError> {
        match self {
            Self::Rig(rig) => Ok(rig),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: URig::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_service(&self) -> Result<&UService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UService::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_service_mut(&mut self) -> Result<&mut UService, ItemKindMatchError> {
        match self {
            Self::Service(service) => Ok(service),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UService::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_ship(&self) -> Result<&UShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UShip::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_ship_mut(&mut self) -> Result<&mut UShip, ItemKindMatchError> {
        match self {
            Self::Ship(ship) => Ok(ship),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UShip::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_skill(&self) -> Result<&USkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USkill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_skill_mut(&mut self) -> Result<&mut USkill, ItemKindMatchError> {
        match self {
            Self::Skill(skill) => Ok(skill),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USkill::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_stance(&self) -> Result<&UStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UStance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_stance_mut(&mut self) -> Result<&mut UStance, ItemKindMatchError> {
        match self {
            Self::Stance(stance) => Ok(stance),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: UStance::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_subsystem(&self) -> Result<&USubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USubsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_subsystem_mut(&mut self) -> Result<&mut USubsystem, ItemKindMatchError> {
        match self {
            Self::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USubsystem::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_sw_effect(&self) -> Result<&USwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    pub(crate) fn get_sw_effect_mut(&mut self) -> Result<&mut USwEffect, ItemKindMatchError> {
        match self {
            Self::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(ItemKindMatchError {
                item_id: self.get_item_id(),
                expected_kind: USwEffect::get_name(),
                actual_kind: self.get_name(),
            }),
        }
    }
    // Service-specific getters
    // TODO: consider moving to service specific item extensions
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_group_id(),
            Self::Booster(booster) => booster.get_group_id(),
            Self::Character(character) => character.get_group_id(),
            Self::Charge(charge) => charge.get_group_id(),
            Self::Drone(drone) => drone.get_group_id(),
            Self::Fighter(fighter) => fighter.get_group_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_group_id(),
            Self::Implant(implant) => implant.get_group_id(),
            Self::Module(module) => module.get_group_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_group_id(),
            Self::Rig(rig) => rig.get_group_id(),
            Self::Service(service) => service.get_group_id(),
            Self::Ship(ship) => ship.get_group_id(),
            Self::Skill(skill) => skill.get_group_id(),
            Self::Stance(stance) => stance.get_group_id(),
            Self::Subsystem(subsystem) => subsystem.get_group_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_group_id(),
        }
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_category_id(),
            Self::Booster(booster) => booster.get_category_id(),
            Self::Character(character) => character.get_category_id(),
            Self::Charge(charge) => charge.get_category_id(),
            Self::Drone(drone) => drone.get_category_id(),
            Self::Fighter(fighter) => fighter.get_category_id(),
            Self::FwEffect(fw_effect) => fw_effect.get_category_id(),
            Self::Implant(implant) => implant.get_category_id(),
            Self::Module(module) => module.get_category_id(),
            Self::ProjEffect(proj_effect) => proj_effect.get_category_id(),
            Self::Rig(rig) => rig.get_category_id(),
            Self::Service(service) => service.get_category_id(),
            Self::Ship(ship) => ship.get_category_id(),
            Self::Skill(skill) => skill.get_category_id(),
            Self::Stance(stance) => stance.get_category_id(),
            Self::Subsystem(subsystem) => subsystem.get_category_id(),
            Self::SwEffect(sw_effect) => sw_effect.get_category_id(),
        }
    }
    pub(crate) fn get_attr(&self, attr_id: &AAttrId) -> Option<AAttrVal> {
        match self.get_attrs() {
            Some(attrs) => attrs.get(attr_id).copied(),
            None => None,
        }
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<AAttrId, AAttrVal>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_attrs(),
            Self::Booster(booster) => booster.get_attrs(),
            Self::Character(character) => character.get_attrs(),
            Self::Charge(charge) => charge.get_attrs(),
            Self::Drone(drone) => drone.get_attrs(),
            Self::Fighter(fighter) => fighter.get_attrs(),
            Self::FwEffect(fw_effect) => fw_effect.get_attrs(),
            Self::Implant(implant) => implant.get_attrs(),
            Self::Module(module) => module.get_attrs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_attrs(),
            Self::Rig(rig) => rig.get_attrs(),
            Self::Service(service) => service.get_attrs(),
            Self::Ship(ship) => ship.get_attrs(),
            Self::Skill(skill) => skill.get_attrs(),
            Self::Stance(stance) => stance.get_attrs(),
            Self::Subsystem(subsystem) => subsystem.get_attrs(),
            Self::SwEffect(sw_effect) => sw_effect.get_attrs(),
        }
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, AItemEffectData>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_effect_datas(),
            Self::Booster(booster) => booster.get_effect_datas(),
            Self::Character(character) => character.get_effect_datas(),
            Self::Charge(charge) => charge.get_effect_datas(),
            Self::Drone(drone) => drone.get_effect_datas(),
            Self::Fighter(fighter) => fighter.get_effect_datas(),
            Self::FwEffect(fw_effect) => fw_effect.get_effect_datas(),
            Self::Implant(implant) => implant.get_effect_datas(),
            Self::Module(module) => module.get_effect_datas(),
            Self::ProjEffect(proj_effect) => proj_effect.get_effect_datas(),
            Self::Rig(rig) => rig.get_effect_datas(),
            Self::Service(service) => service.get_effect_datas(),
            Self::Ship(ship) => ship.get_effect_datas(),
            Self::Skill(skill) => skill.get_effect_datas(),
            Self::Stance(stance) => stance.get_effect_datas(),
            Self::Subsystem(subsystem) => subsystem.get_effect_datas(),
            Self::SwEffect(sw_effect) => sw_effect.get_effect_datas(),
        }
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_defeff_key(),
            Self::Booster(booster) => booster.get_defeff_key(),
            Self::Character(character) => character.get_defeff_key(),
            Self::Charge(charge) => charge.get_defeff_key(),
            Self::Drone(drone) => drone.get_defeff_key(),
            Self::Fighter(fighter) => fighter.get_defeff_key(),
            Self::FwEffect(fw_effect) => fw_effect.get_defeff_key(),
            Self::Implant(implant) => implant.get_defeff_key(),
            Self::Module(module) => module.get_defeff_key(),
            Self::ProjEffect(proj_effect) => proj_effect.get_defeff_key(),
            Self::Rig(rig) => rig.get_defeff_key(),
            Self::Service(service) => service.get_defeff_key(),
            Self::Ship(ship) => ship.get_defeff_key(),
            Self::Skill(skill) => skill.get_defeff_key(),
            Self::Stance(stance) => stance.get_defeff_key(),
            Self::Subsystem(subsystem) => subsystem.get_defeff_key(),
            Self::SwEffect(sw_effect) => sw_effect.get_defeff_key(),
        }
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_skill_reqs(),
            Self::Booster(booster) => booster.get_skill_reqs(),
            Self::Character(character) => character.get_skill_reqs(),
            Self::Charge(charge) => charge.get_skill_reqs(),
            Self::Drone(drone) => drone.get_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_skill_reqs(),
            Self::FwEffect(fw_effect) => fw_effect.get_skill_reqs(),
            Self::Implant(implant) => implant.get_skill_reqs(),
            Self::Module(module) => module.get_skill_reqs(),
            Self::ProjEffect(proj_effect) => proj_effect.get_skill_reqs(),
            Self::Rig(rig) => rig.get_skill_reqs(),
            Self::Service(service) => service.get_skill_reqs(),
            Self::Ship(ship) => ship.get_skill_reqs(),
            Self::Skill(skill) => skill.get_skill_reqs(),
            Self::Stance(stance) => stance.get_skill_reqs(),
            Self::Subsystem(subsystem) => subsystem.get_skill_reqs(),
            Self::SwEffect(sw_effect) => sw_effect.get_skill_reqs(),
        }
    }
    pub(crate) fn get_effective_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(booster) => booster.get_skill_reqs(),
            Self::Character(_) => None,
            Self::Charge(charge) => charge.get_skill_reqs(),
            Self::Drone(drone) => drone.get_skill_reqs(),
            Self::Fighter(fighter) => fighter.get_skill_reqs(),
            Self::FwEffect(_) => None,
            Self::Implant(implant) => implant.get_skill_reqs(),
            Self::Module(module) => module.get_skill_reqs(),
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Service(_) => None,
            Self::Ship(ship) => ship.get_skill_reqs(),
            Self::Skill(skill) => skill.get_skill_reqs(),
            Self::Stance(_) => None,
            Self::Subsystem(subsystem) => subsystem.get_skill_reqs(),
            Self::SwEffect(_) => None,
        }
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        match self {
            Self::Autocharge(autocharge) => autocharge.get_axt(),
            Self::Booster(booster) => booster.get_axt(),
            Self::Character(character) => character.get_axt(),
            Self::Charge(charge) => charge.get_axt(),
            Self::Drone(drone) => drone.get_axt(),
            Self::Fighter(fighter) => fighter.get_axt(),
            Self::FwEffect(fw_effect) => fw_effect.get_axt(),
            Self::Implant(implant) => implant.get_axt(),
            Self::Module(module) => module.get_axt(),
            Self::ProjEffect(proj_effect) => proj_effect.get_axt(),
            Self::Rig(rig) => rig.get_axt(),
            Self::Service(service) => service.get_axt(),
            Self::Ship(ship) => ship.get_axt(),
            Self::Skill(skill) => skill.get_axt(),
            Self::Stance(stance) => stance.get_axt(),
            Self::Subsystem(subsystem) => subsystem.get_axt(),
            Self::SwEffect(sw_effect) => sw_effect.get_axt(),
        }
    }
}
impl Named for UItem {
    fn get_name() -> &'static str {
        "Item"
    }
}
impl GetId<ItemId> for UItem {
    fn get_id(&self) -> ItemId {
        self.get_item_id()
    }
}
