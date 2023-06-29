use crate::{
    cmd::{fit, item},
    shared::HState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddFighterCmd,
}
impl HAddFighterCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddFighterCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::SsFitId {
        self.fit_id
    }
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> &HState {
        self.fit_cmd.get_state()
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd::ss) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeFighterCmd) -> Self {
        Self {
            fit_cmd: fit::HChangeFighterCmd::from_item_cmd(item_id, item_cmd),
        }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.fit_cmd.get_item_id()
    }
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.fit_cmd.get_state()
    }
}
impl From<fit::HChangeFighterCmd> for HChangeFighterCmd {
    fn from(fit_cmd: fit::HChangeFighterCmd) -> Self {
        Self { fit_cmd }
    }
}
