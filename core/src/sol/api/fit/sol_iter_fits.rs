use crate::sol::{
    SolarSystem,
    api::{Fit, FitMut, MutIter},
};

impl SolarSystem {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit<'_>> {
        self.u_data.fits.keys().map(|fit_key| Fit::new(self, fit_key))
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMut<'_>> {
        let fit_keys = self.u_data.fits.keys().collect();
        MutIter::new(self, fit_keys)
    }
}
