use crate::{
    sol::svc::svce_calc::{
        misc::SolAttrValData,
        rah::SolRahSim,
        registers::{
            SolBuffRegister, SolDependencyRegister, SolProjectionRegister, SolRevisionRegister, SolStandardRegister,
        },
    },
    src::Src,
};

#[derive(Clone)]
pub(in crate::sol::svc) struct SolSvcCalcData {
    pub(in crate::sol::svc::svce_calc) attrs: SolAttrValData,
    pub(in crate::sol::svc::svce_calc) std: SolStandardRegister,
    pub(in crate::sol::svc::svce_calc) buffs: SolBuffRegister,
    pub(in crate::sol::svc::svce_calc) deps: SolDependencyRegister,
    pub(in crate::sol::svc::svce_calc) revs: SolRevisionRegister,
    pub(in crate::sol::svc::svce_calc) projs: SolProjectionRegister,
    pub(in crate::sol::svc::svce_calc) rah: SolRahSim,
}
impl SolSvcCalcData {
    pub(in crate::sol::svc) fn new(src: &Src) -> Self {
        Self {
            attrs: SolAttrValData::new(),
            std: SolStandardRegister::new(),
            buffs: SolBuffRegister::new(),
            deps: SolDependencyRegister::new(),
            revs: SolRevisionRegister::new(),
            projs: SolProjectionRegister::new(),
            rah: SolRahSim::new(src),
        }
    }
}
