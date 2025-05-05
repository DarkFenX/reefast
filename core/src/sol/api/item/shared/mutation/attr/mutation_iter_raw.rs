use lender::{Lender, Lending};

use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem,
        api::{
            EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut,
            RawMAttr, RawMAttrMut,
        },
    },
};

impl<'a> Mutation<'a> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr> {
        let (sol, item_key) = match self {
            Self::Effective(effective_mutation) => (effective_mutation.sol, effective_mutation.item_key),
            Self::Incomplete(incomplete_mutation) => (incomplete_mutation.sol, incomplete_mutation.item_key),
        };
        iter_raw_mattrs(sol, item_key)
    }
}

impl<'a> MutationMut<'a> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr> {
        let (sol, item_key) = match self {
            Self::Effective(effective_mutation) => (&*effective_mutation.sol, effective_mutation.item_key),
            Self::Incomplete(incomplete_mutation) => (&*incomplete_mutation.sol, incomplete_mutation.item_key),
        };
        iter_raw_mattrs(sol, item_key)
    }
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs_mut(&mut self) -> RawMAttrIter {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.iter_raw_mattrs_mut(),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.iter_raw_mattrs_mut(),
        }
    }
}

impl<'a> EffectiveMutation<'a> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr> {
        iter_raw_mattrs(self.sol, self.item_key)
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr> {
        iter_raw_mattrs(self.sol, self.item_key)
    }
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs_mut(&mut self) -> RawMAttrIter {
        RawMAttrIter::new(self.sol, self.item_key)
    }
}

impl<'a> IncompleteMutation<'a> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr> {
        iter_raw_mattrs(self.sol, self.item_key)
    }
}

impl<'a> IncompleteMutationMut<'a> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr> {
        iter_raw_mattrs(self.sol, self.item_key)
    }
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs_mut(&mut self) -> RawMAttrIter {
        RawMAttrIter::new(self.sol, self.item_key)
    }
}

// Lending iterator for attribute rolls
pub struct RawMAttrIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_key: ItemKey,
    a_attr_ids: Vec<ad::AAttrId>,
    index: usize,
}
impl<'iter> RawMAttrIter<'iter> {
    pub(in crate::sol::api) fn new(sol: &'iter mut SolarSystem, item_key: ItemKey) -> Self {
        let a_attr_ids = raw_mutated_a_attr_id_iter(sol, item_key).collect();
        Self {
            sol,
            item_key,
            a_attr_ids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for RawMAttrIter<'iter> {
    type Lend = RawMAttrMut<'lend>;
}
impl<'iter> Lender for RawMAttrIter<'iter> {
    fn next(&mut self) -> Option<RawMAttrMut> {
        let a_attr_id = *self.a_attr_ids.get(self.index)?;
        self.index += 1;
        Some(RawMAttrMut::new(self.sol, self.item_key, a_attr_id))
    }
}

fn raw_mutated_a_attr_id_iter(sol: &SolarSystem, item_key: ItemKey) -> impl ExactSizeIterator<Item = ad::AAttrId> {
    sol.uad
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .keys()
        .copied()
}

fn iter_raw_mattrs(sol: &SolarSystem, item_key: ItemKey) -> impl ExactSizeIterator<Item = RawMAttr> + use<'_> {
    raw_mutated_a_attr_id_iter(sol, item_key).map(move |a_attr_id| RawMAttr::new(sol, item_key, a_attr_id))
}
