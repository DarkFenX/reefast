use crate::{
    ss::item::{Item, Skill},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_skill(&self, item_id: &ReeId) -> Result<&Skill> {
        match self.get_item(item_id)? {
            Item::Skill(s) => Ok(s),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected Skill as item with ID {item_id}"),
            )),
        }
    }
    pub fn get_skills(&self, fit_id: ReeId) -> Vec<&Skill> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Skill(s) if s.fit_id == fit_id => Some(s),
                _ => None,
            })
            .collect()
    }
    pub fn add_skill(&mut self, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Result<ReeId> {
        check_skill_level(level)?;
        let item_id = self.alloc_item_id()?;
        let skill = Item::Skill(Skill::new(&self.src, item_id, fit_id, type_id, level));
        self.add_item(skill);
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
