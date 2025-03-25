use crate::sol::ItemId;

#[derive(Debug)]
pub struct ProjNotFoundError {
    pub projector_item_id: ItemId,
    pub projectee_item_id: ItemId,
}
impl ProjNotFoundError {
    pub(crate) fn new(projector_item_id: ItemId, projectee_item_id: ItemId) -> Self {
        Self {
            projector_item_id,
            projectee_item_id,
        }
    }
}
impl std::error::Error for ProjNotFoundError {}
impl std::fmt::Display for ProjNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "projection {}->{} is already defined",
            self.projector_item_id, self.projectee_item_id
        )
    }
}
