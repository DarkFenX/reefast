use crate::{
    defs::{EAttrId, SolItemId},
    sol::{
        svc::{err::AttrCalcError, svce_calc::SolAttrVal, SolSvcs},
        SolView,
    },
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolItemAttrValData {
    pub(in crate::sol::svc::svce_calc) values: StMap<EAttrId, SolAttrVal>,
    pub(in crate::sol::svc::svce_calc) overrides:
        StMap<EAttrId, fn(&mut SolSvcs, &SolView, &SolItemId) -> Result<SolAttrVal, AttrCalcError>>,
}
impl SolItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            overrides: StMap::new(),
        }
    }
}
