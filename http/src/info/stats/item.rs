use crate::{
    info::stats::details::{HStatLayerEhp, HStatLayerHp, HStatLayerResist, HStatTank},
    util::TriStateField,
};

#[derive(serde::Serialize)]
pub(crate) struct HItemStats {
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<HStatTank<HStatLayerHp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ehp: Option<Vec<Option<HStatTank<HStatLayerEhp>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<HStatTank<HStatLayerEhp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) resists: TriStateField<HStatTank<HStatLayerResist>>,
}
impl HItemStats {
    pub fn new() -> Self {
        Self {
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            speed: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: Option::default(),
            wc_ehp: TriStateField::default(),
            resists: TriStateField::default(),
        }
    }
}
