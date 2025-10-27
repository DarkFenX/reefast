use crate::{
    def::AttrVal,
    sol::api::{FitCharacterStatError, FitMut, ItemMutCommon},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_drone_control_range(&mut self) -> Result<AttrVal, FitCharacterStatError> {
        Ok(self.get_character_for_stats()?.get_stat_drone_control_range()?)
    }
}
