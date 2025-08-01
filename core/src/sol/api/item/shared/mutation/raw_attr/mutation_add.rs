use crate::{
    ad,
    def::AttrId,
    err::basic::ItemMAttrNotFoundError,
    sol::{
        SolarSystem,
        api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut, RawMAttrMut},
    },
    ud::UItemKey,
    util::UnitInterval,
};

impl<'a> MutationMut<'a> {
    /// Mutate an attribute with a roll quality.
    ///
    /// Accepts roll of any attribute, even if it is not defined by item mutator. In this case, roll
    /// will be stored, and its effect won't be applied.
    pub fn mutate_raw(&mut self, attr_id: AttrId, roll: UnitInterval) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.mutate_raw(attr_id, roll),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.mutate_raw(attr_id, roll),
        }
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Mutate an attribute with a roll quality.
    ///
    /// Accepts roll of any attribute, even if it is not defined by item mutator. In this case, roll
    /// will be stored, and its effect won't be applied.
    pub fn mutate_raw(&mut self, attr_id: AttrId, roll: UnitInterval) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
        mutate_raw(self.sol, self.item_key, attr_id, roll)
    }
}

impl<'a> IncompleteMutationMut<'a> {
    /// Mutate an attribute with a roll quality.
    ///
    /// Accepts roll of any attribute, even if it is not defined by item mutator. In this case, roll
    /// will be stored, and its effect won't be applied.
    pub fn mutate_raw(&mut self, attr_id: AttrId, roll: UnitInterval) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
        mutate_raw(self.sol, self.item_key, attr_id, roll)
    }
}

fn mutate_raw(
    sol: &mut SolarSystem,
    item_key: UItemKey,
    a_attr_id: ad::AAttrId,
    roll: UnitInterval,
) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
    match sol
        .u_data
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&a_attr_id)
    {
        Some(_) => Err(ItemMAttrNotFoundError {
            item_id: sol.u_data.items.id_by_key(item_key),
            attr_id: a_attr_id,
        }
        .into()),
        None => {
            let mut raw_mattr = RawMAttrMut::new(sol, item_key, a_attr_id);
            raw_mattr.set_roll(roll);
            Ok(raw_mattr)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AttrMutateRawError {
    #[error("{0}")]
    AlreadyMutated(#[from] ItemMAttrNotFoundError),
}
