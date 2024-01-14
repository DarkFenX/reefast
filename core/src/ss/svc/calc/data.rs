use super::support::{AttrCapData, AttrValData, ModRegister, ProjRegister};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::calc) attrs: AttrValData,
    pub(in crate::ss::svc::calc) caps: AttrCapData,
    pub(in crate::ss::svc::calc) mods: ModRegister,
    pub(in crate::ss::svc::calc) projs: ProjRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: AttrValData::new(),
            caps: AttrCapData::new(),
            mods: ModRegister::new(),
            projs: ProjRegister::new(),
        }
    }
}
