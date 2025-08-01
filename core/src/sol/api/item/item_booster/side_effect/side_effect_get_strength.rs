use itertools::Itertools;

use crate::{
    ad::AEffectId,
    def::{AttrId, AttrVal},
    misc::OpInfo,
    sol::api::{FullSideEffect, FullSideEffectMut},
    src::Src,
};

pub struct SideEffectPartialStr {
    op: OpInfo,
    attr_id: AttrId,
}
impl SideEffectPartialStr {
    pub fn get_op(&self) -> OpInfo {
        self.op
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.attr_id
    }
}

pub struct SideEffectStr {
    op: OpInfo,
    attr_id: AttrId,
    value: AttrVal,
}
impl SideEffectStr {
    pub fn get_op(&self) -> OpInfo {
        self.op
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.attr_id
    }
    pub fn get_value(&self) -> AttrVal {
        self.value
    }
}

impl<'a> FullSideEffect<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.u_data.src, &self.effect_id)
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.u_data.src, &self.effect_id)
    }
    /// Get side effect strength as an operator and modification value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength(&mut self) -> Option<SideEffectStr> {
        match self.get_strength_partial() {
            Some(partial) => match self.sol.internal_get_item_attr(self.key, &partial.attr_id) {
                Ok(calc_val) => Some(SideEffectStr {
                    op: partial.op,
                    attr_id: partial.attr_id,
                    value: calc_val.extra,
                }),
                Err(_) => None,
            },
            None => None,
        }
    }
}

fn get_strength_partial(src: &Src, effect_id: &AEffectId) -> Option<SideEffectPartialStr> {
    let effect_key = src.get_effect_key_by_id(effect_id).unwrap();
    let mut se_strs = src
        .get_effect(effect_key)
        .get_mods()
        .iter()
        .map(|a_modifier| (a_modifier.op, a_modifier.affector_attr_id))
        .collect_vec();
    match se_strs.len() {
        0 => None,
        1 => se_strs
            .into_iter()
            .map(|(a_op, a_attr_id)| SideEffectPartialStr {
                op: a_op.into(),
                attr_id: a_attr_id,
            })
            .next(),
        _ => {
            let (base_a_op, base_a_attr_id) = se_strs.pop().unwrap();
            match se_strs
                .into_iter()
                .all(|(a_op, a_attr_id)| a_op == base_a_op && a_attr_id == base_a_attr_id)
            {
                true => Some(SideEffectPartialStr {
                    op: base_a_op.into(),
                    attr_id: base_a_attr_id,
                }),
                false => None,
            }
        }
    }
}
