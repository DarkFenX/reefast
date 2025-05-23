use crate::{
    sol::{reffs::REffs, rprojs::RProjs, svc::Svc, uad::Uad},
    src::Src,
};

// Solar system glues everything together and is actual "god object" of the lib. It holds all the
// data and exposes interface to manipulate, process and fetch it.
#[derive(Clone)]
pub struct SolarSystem {
    pub(in crate::sol) uad: Uad,
    pub(in crate::sol) svc: Svc,
    pub(in crate::sol) reffs: REffs,
    pub(in crate::sol) rprojs: RProjs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        let svcs = Svc::new(&src);
        Self {
            uad: Uad::new(src),
            svc: svcs,
            reffs: REffs::new(),
            rprojs: RProjs::new(),
        }
    }
}
