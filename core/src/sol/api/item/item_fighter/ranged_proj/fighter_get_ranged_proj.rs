use crate::{
    def::ItemId,
    sol::api::{Fighter, FighterMut, GetRangedProjError, RangedProj, RangedProjMut},
};

impl<'a> Fighter<'a> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<RangedProj<'_>, GetRangedProjError> {
        self.sol.internal_get_ranged_proj(self.key, projectee_item_id)
    }
}

impl<'a> FighterMut<'a> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<RangedProj<'_>, GetRangedProjError> {
        self.sol.internal_get_ranged_proj(self.key, projectee_item_id)
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<RangedProjMut<'_>, GetRangedProjError> {
        self.sol.internal_get_ranged_proj_mut(self.key, projectee_item_id)
    }
}
