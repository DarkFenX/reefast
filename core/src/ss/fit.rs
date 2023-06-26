use std::collections::HashSet;

use crate::{
    consts::ModRack,
    defs::{SsFitId, SsItemId},
    ssi::SsItem,
};

pub(crate) struct SsFit {
    pub(crate) id: SsFitId,
    pub(crate) character: Option<SsItemId>,
    pub(crate) skills: HashSet<SsItemId>,
    pub(crate) implants: HashSet<SsItemId>,
    pub(crate) boosters: HashSet<SsItemId>,
    pub(crate) ship: Option<SsItemId>,
    pub(crate) stance: Option<SsItemId>,
    pub(crate) subsystems: HashSet<SsItemId>,
    pub(crate) mods_high: HashSet<SsItemId>,
    pub(crate) mods_mid: HashSet<SsItemId>,
    pub(crate) mods_low: HashSet<SsItemId>,
    pub(crate) rigs: HashSet<SsItemId>,
    pub(crate) drones: HashSet<SsItemId>,
    pub(crate) fighters: HashSet<SsItemId>,
}
impl SsFit {
    pub(in crate::ss) fn new(id: SsFitId) -> Self {
        Self {
            id,
            character: None,
            skills: HashSet::new(),
            implants: HashSet::new(),
            boosters: HashSet::new(),
            ship: None,
            stance: None,
            subsystems: HashSet::new(),
            mods_high: HashSet::new(),
            mods_mid: HashSet::new(),
            mods_low: HashSet::new(),
            rigs: HashSet::new(),
            drones: HashSet::new(),
            fighters: HashSet::new(),
        }
    }
    pub(in crate::ss) fn add_item(&mut self, item: &SsItem) {
        match item {
            SsItem::Character(character) => self.character = Some(character.id),
            SsItem::Skill(skill) => {
                self.skills.insert(skill.id);
                ()
            }
            SsItem::Implant(implant) => {
                self.implants.insert(implant.id);
                ()
            }
            SsItem::Booster(booster) => {
                self.boosters.insert(booster.id);
                ()
            }
            SsItem::Ship(ship) => self.ship = Some(ship.id),
            SsItem::Stance(stance) => self.stance = Some(stance.id),
            SsItem::Subsystem(subsystem) => {
                self.subsystems.insert(subsystem.id);
                ()
            }
            SsItem::Module(module) => {
                match module.rack {
                    ModRack::High => self.mods_high.insert(module.id),
                    ModRack::Mid => self.mods_mid.insert(module.id),
                    ModRack::Low => self.mods_low.insert(module.id),
                };
                ()
            }
            SsItem::Rig(rig) => {
                self.rigs.insert(rig.id);
                ()
            }
            SsItem::Drone(drone) => {
                self.drones.insert(drone.id);
                ()
            }
            SsItem::Fighter(fighter) => {
                self.fighters.insert(fighter.id);
                ()
            }
            // Ignore charges and system-wide effects
            _ => (),
        };
    }
    pub(in crate::ss) fn remove_item(&mut self, item: &SsItem) {
        match item {
            SsItem::Character(character) if self.character == Some(character.id) => self.character = None,
            SsItem::Skill(skill) => {
                self.skills.remove(&skill.id);
                ()
            }
            SsItem::Implant(implant) => {
                self.implants.remove(&implant.id);
                ()
            }
            SsItem::Booster(booster) => {
                self.boosters.remove(&booster.id);
                ()
            }
            SsItem::Ship(ship) if self.ship == Some(ship.id) => self.ship = None,
            SsItem::Stance(stance) if self.stance == Some(stance.id) => self.stance = None,
            SsItem::Subsystem(subsystem) => {
                self.subsystems.remove(&subsystem.id);
                ()
            }
            SsItem::Module(module) => {
                match module.rack {
                    ModRack::High => self.mods_high.remove(&module.id),
                    ModRack::Mid => self.mods_mid.remove(&module.id),
                    ModRack::Low => self.mods_low.remove(&module.id),
                };
                ()
            }
            SsItem::Rig(rig) => {
                self.rigs.remove(&rig.id);
                ()
            }
            SsItem::Drone(drone) => {
                self.drones.remove(&drone.id);
            }
            SsItem::Fighter(fighter) => {
                self.fighters.remove(&fighter.id);
                ()
            }
            // Ignore charges and system-wide effects
            _ => (),
        }
    }
}
