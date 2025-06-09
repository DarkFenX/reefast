use detailed::{HFitValResultDetailed, HSolValResultDetailed};
use simple::HValResultSimple;

mod detailed;
mod details;
mod simple;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSolValResult {
    Simple(HValResultSimple),
    Detailed(Box<HSolValResultDetailed>),
}
impl From<bool> for HSolValResult {
    fn from(core_result: bool) -> Self {
        Self::Simple(core_result.into())
    }
}
impl From<&rc::val::ValResultSol> for HSolValResult {
    fn from(core_result: &rc::val::ValResultSol) -> Self {
        Self::Detailed(Box::new(core_result.into()))
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFitValResult {
    Simple(HValResultSimple),
    Detailed(Box<HFitValResultDetailed>),
}
impl From<bool> for HFitValResult {
    fn from(core_result: bool) -> Self {
        Self::Simple(core_result.into())
    }
}
impl From<&rc::val::ValResultFit> for HFitValResult {
    fn from(core_result: &rc::val::ValResultFit) -> Self {
        Self::Detailed(Box::new(core_result.into()))
    }
}
