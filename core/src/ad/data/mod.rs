pub use attr::AAttr;
pub use buff::{ABuff, ABuffAttrMod};
pub use effect::{AAttrMod, AEffect};
pub use item::{AItem, AItemEffData};
pub use muta::{AMuta, AMutaAttrRange};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

/// Adapted data storage.
pub struct AData {
    pub items: Vec<AItem>,
    pub attrs: Vec<AAttr>,
    pub mutas: Vec<AMuta>,
    pub effects: Vec<AEffect>,
    pub buffs: Vec<ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            attrs: Vec::new(),
            mutas: Vec::new(),
            effects: Vec::new(),
            buffs: Vec::new(),
        }
    }
}
