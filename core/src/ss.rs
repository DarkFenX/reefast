use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    consts::State,
    src::Src,
    ssi::{Booster, Character, Charge, Implant, Item, Module, Rig, Ship, Skill, Stance, Subsystem, SwEffect},
    Error, ErrorKind, ReeId, ReeIdx, ReeInt, Result,
};

pub struct SolarSystem {
    src: Arc<Src>,
    fit_cnt: Wrapping<ReeId>,
    fits: HashSet<ReeId>,
    // fleet_cnt: ReeId,
    // fleets: HashMap<ReeId, Fleet>,
    item_cnt: Wrapping<ReeId>,
    items: HashMap<ReeId, Item>,
}
impl SolarSystem {
    pub fn new(src: Arc<Src>) -> SolarSystem {
        SolarSystem {
            src,
            fit_cnt: Wrapping(0),
            fits: HashSet::new(),
            item_cnt: Wrapping(0),
            items: HashMap::new(),
        }
    }
    pub fn set_src(&mut self, src: Arc<Src>) {
        for item in self.items.values_mut() {
            item.reload_cached_item(&src)
        }
        self.src = src;
    }
    // Fit methods
    pub fn add_fit(&mut self) -> Result<ReeId> {
        let fit_id = self.alloc_fit_id()?;
        self.fits.insert(fit_id);
        Ok(fit_id)
    }
    pub fn remove_fit(&mut self, fit_id: ReeId) -> bool {
        self.items.drain_filter(|_, v| v.get_fit_id() == Some(fit_id));
        self.fits.remove(&fit_id)
    }
    fn alloc_fit_id(&mut self) -> Result<ReeId> {
        let start = self.fit_cnt;
        while self.fits.contains(&self.fit_cnt.0) {
            self.fit_cnt += 1;
            if start == self.fit_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate fit ID"));
            }
        }
        Ok(self.fit_cnt.0)
    }
    // Character methods
    pub fn get_character(&self, fit_id: ReeId) -> Option<ReeId> {
        self.items
            .values()
            .find_or_first(|v| match v {
                Item::Character(c) if c.fit_id == fit_id => true,
                _ => false,
            })
            .map(|v| v.get_id())
    }
    pub fn set_character(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_character(fit_id)?;
        let item_id = self.alloc_item_id()?;
        let character = Item::Character(Character::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, character);
        Ok(item_id)
    }
    pub fn remove_character(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Character(c) if c.fit_id == fit_id => true,
                _ => false,
            })
            .collect_vec();
        Ok(!removed.is_empty())
    }
    // Ship methods
    pub fn get_ship(&self, fit_id: ReeId) -> Option<ReeId> {
        self.items
            .values()
            .find_or_first(|v| match v {
                Item::Ship(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .map(|v| v.get_id())
    }
    pub fn set_ship(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_ship(fit_id)?;
        let item_id = self.alloc_item_id()?;
        let ship = Item::Ship(Ship::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, ship);
        Ok(item_id)
    }
    pub fn remove_ship(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Ship(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .collect_vec();
        Ok(!removed.is_empty())
    }
    // Stance methods
    pub fn get_stance(&self, fit_id: ReeId) -> Option<ReeId> {
        self.items
            .values()
            .find_or_first(|v| match v {
                Item::Stance(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .map(|v| v.get_id())
    }
    pub fn set_stance(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_stance(fit_id)?;
        let item_id = self.alloc_item_id()?;
        let stance = Item::Stance(Stance::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, stance);
        Ok(item_id)
    }
    pub fn remove_stance(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Stance(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .collect_vec();
        Ok(!removed.is_empty())
    }
    // Subsystem methods
    pub fn get_subsystems(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Subsystem(s) if s.fit_id == fit_id => Some(s.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_subsystem(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let subsystem = Item::Subsystem(Subsystem::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, subsystem);
        Ok(item_id)
    }
    // Module methods
    pub fn get_modules_high(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleHigh(m) if m.fit_id == fit_id => Some(m.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_module_high(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        pos: ReeIdx,
        charge_type_id: Option<ReeInt>,
    ) -> Result<(ReeId, Option<ReeId>)> {
        match self.items.values().find_or_first(|v| match v {
            Item::ModuleHigh(m) if m.fit_id == fit_id && m.pos == pos => true,
            _ => false,
        }) {
            Some(i) => {
                return Err(Error::new(
                    ErrorKind::SlotTaken,
                    format!("high slot position {} is taken by item ID {}", pos, i.get_id()),
                ))
            }
            _ => (),
        }
        let item_id = self.alloc_item_id()?;
        let charge_id = match charge_type_id {
            Some(i) => {
                let charge_id = self.alloc_item_id()?;
                let charge = Item::Charge(Charge::new(&self.src, charge_id, fit_id, i, item_id));
                self.items.insert(charge_id, charge);
                Some(charge_id)
            }
            None => None,
        };
        let module = Item::ModuleHigh(Module::new(&self.src, item_id, fit_id, type_id, state, pos, charge_id));
        self.items.insert(item_id, module);
        Ok((item_id, charge_id))
    }
    // Rig methods
    pub fn get_rigs(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Rig(r) if r.fit_id == fit_id => Some(r.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_rig(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let rig = Item::Rig(Rig::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, rig);
        Ok(item_id)
    }
    pub fn get_rig_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Rig(r) => Ok(r.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Rig as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_rig_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Rig(r) => r.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Rig as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    // Skill methods
    pub fn get_skills(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Skill(s) if s.fit_id == fit_id => Some(s.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_skill(&mut self, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Result<ReeId> {
        check_skill_level(level)?;
        let item_id = self.alloc_item_id()?;
        let skill = Item::Skill(Skill::new(&self.src, item_id, fit_id, type_id, level));
        self.items.insert(item_id, skill);
        Ok(item_id)
    }
    pub fn set_skill_level(&mut self, item_id: &ReeId, level: ReeInt) -> Result<()> {
        check_skill_level(level)?;
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Skill(s) => s.level = level,
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Skill as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    // Implant methods
    pub fn get_implants(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Implant(i) if i.fit_id == fit_id => Some(i.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_implant(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let implant = Item::Implant(Implant::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, implant);
        Ok(item_id)
    }
    pub fn get_implant_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Implant(i) => Ok(i.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Implant as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_implant_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Implant(i) => i.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Implant as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    // Booster methods
    pub fn get_boosters(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Booster(b) if b.fit_id == fit_id => Some(b.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_booster(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let booster = Item::Booster(Booster::new(&self.src, item_id, fit_id, type_id));
        self.items.insert(item_id, booster);
        Ok(item_id)
    }
    pub fn get_booster_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Booster(b) => Ok(b.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Booster as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_booster_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Booster(b) => b.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Booster as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    // System-wide effect methods
    pub fn get_sw_effects(&self) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::SwEffect(e) => Some(e.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_sw_effect(&mut self, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let sw_effect = Item::SwEffect(SwEffect::new(&self.src, item_id, type_id));
        self.items.insert(item_id, sw_effect);
        Ok(item_id)
    }
    pub fn get_sw_effect_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::SwEffect(e) => Ok(e.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected SwEffect as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_sw_effect_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::SwEffect(e) => e.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected SwEffect as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    // General
    fn alloc_item_id(&mut self) -> Result<ReeId> {
        let start = self.item_cnt;
        while self.items.contains_key(&self.item_cnt.0) {
            self.item_cnt += 1;
            if start == self.item_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate item ID"));
            }
        }
        Ok(self.item_cnt.0)
    }
    pub fn remove_item(&mut self, item_id: &ReeId) -> bool {
        match self.items.remove(item_id) {
            None => false,
            Some(main) => {
                match main {
                    // Remove reference to charge if it's charge which we're removing
                    Item::Charge(c) => match self.items.get_mut(&c.cont) {
                        None => return true,
                        Some(other) => match other {
                            Item::ModuleHigh(m) => m.charge = None,
                            Item::ModuleMid(m) => m.charge = None,
                            Item::ModuleLow(m) => m.charge = None,
                            _ => (),
                        },
                    },
                    // Remove charge if we're removing a module, charges cannot exist without their carrier
                    Item::ModuleHigh(m) => match m.charge {
                        None => (),
                        Some(other_id) => {
                            self.items.remove(&other_id);
                            ()
                        }
                    },
                    Item::ModuleMid(m) => match m.charge {
                        None => (),
                        Some(other_id) => {
                            self.items.remove(&other_id);
                            ()
                        }
                    },
                    Item::ModuleLow(m) => match m.charge {
                        None => (),
                        Some(other_id) => {
                            self.items.remove(&other_id);
                            ()
                        }
                    },
                    _ => (),
                };
                true
            }
        }
    }
}

fn check_skill_level(level: ReeInt) -> Result<()> {
    if level > 5 || level < 0 {
        return Err(Error::new(
            ErrorKind::SkillLevelRange,
            format!("skill level must be 0..5, got {level}"),
        ));
    };
    Ok(())
}
