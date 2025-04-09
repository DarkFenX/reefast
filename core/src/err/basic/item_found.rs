use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemFoundError {
    pub item_id: ItemId,
}
impl std::error::Error for ItemFoundError {}
impl std::fmt::Display for ItemFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} not found", self.item_id)
    }
}
impl From<ItemId> for ItemFoundError {
    fn from(item_id: ItemId) -> Self {
        Self { item_id }
    }
}
