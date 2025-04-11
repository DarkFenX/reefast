use crate::sol::{SolarSystem, info::FitInfo};

impl SolarSystem {
    pub fn get_fit_infos(&self) -> Vec<FitInfo> {
        self.uad
            .fits
            .values()
            .map(|fit| FitInfo::from_fit(&self.uad, fit))
            .collect()
    }
}
