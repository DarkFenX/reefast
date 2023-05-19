use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::{
        info::FighterInfo,
        item::{Fighter, Item},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fighter_info(&self, item_id: &ReeId) -> Result<FighterInfo> {
        Ok(self.get_fighter(item_id)?.into())
    }
    pub fn get_fit_fighter_infos(&self, fit_id: &ReeId) -> Vec<FighterInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Fighter(f) if f.fit_id == *fit_id => Some(f.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_fighter(&mut self, fit_id: ReeId, type_id: ReeInt, state: State) -> Result<FighterInfo> {
        let item_id = self.alloc_item_id()?;
        let fighter = Fighter::new(&self.src, item_id, fit_id, type_id, state);
        let info = FighterInfo::from(&fighter);
        let item = Item::Fighter(fighter);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fighter_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_fighter_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    fn get_fighter(&self, item_id: &ReeId) -> Result<&Fighter> {
        let item = self.get_item(item_id)?;
        match item {
            Item::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Fighter::get_name(),
            ))),
        }
    }
    fn get_fighter_mut(&mut self, item_id: &ReeId) -> Result<&mut Fighter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            Item::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Fighter::get_name(),
            ))),
        }
    }
}
