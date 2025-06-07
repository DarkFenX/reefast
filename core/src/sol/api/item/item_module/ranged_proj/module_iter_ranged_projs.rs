use crate::sol::api::{Module, ModuleMut, RangedProj, RangedProjIter, iter_ranged_projs};

impl<'a> Module<'a> {
    /// Iterates over module's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.key)
    }
}

impl<'a> ModuleMut<'a> {
    /// Iterates over module's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.key)
    }
    /// Iterates over module's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter<'_> {
        RangedProjIter::new(self.sol, self.key)
    }
}
