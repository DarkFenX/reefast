use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, EMutaId, MutaRoll, SkillLevel, SolItemId},
    err::basic::{ItemLoadedError, ItemMutatedError, ItemNotMutatedError},
    sol::item::{SolEffectModes, SolItemAttrMutation, SolItemBase, SolItemMutation, SolItemState},
    src::Src,
    util::StMap,
};

// Item mutable base stores all the data every mutable item should have.
//
// Mutated item can have 3 states:
// - Non-mutated - mutation is not set, only base item info is used
// - Mutated, mutation is loaded - source had all the needed mutation data, which was processed and
// stored on cache. In this case, item base stores mutated item type, and base item type ID is
// stored on mutation cache;
// - Mutated, mutation not loaded - item base stores base item type, mutation stores mutator ID and
// attribute mutations, and mutation cache isn't set.
#[derive(Clone)]
pub(in crate::sol::item) struct SolItemBaseMutable {
    base: SolItemBase,
    mutation: Option<SolItemMutationData>,
}
impl SolItemBaseMutable {
    pub(in crate::sol::item) fn new(
        src: &Src,
        id: SolItemId,
        type_id: EItemId,
        state: SolItemState,
        mutation_request: Option<SolItemMutation>,
    ) -> Self {
        let mutation_request = match mutation_request {
            Some(mutation_request) => mutation_request,
            // No mutation - regular non-mutated item setup
            None => {
                return Self {
                    base: SolItemBase::new(src, id, type_id, state),
                    mutation: None,
                }
            }
        };
        let base_a_item = match src.get_a_item(&type_id) {
            Some(base_a_item) => base_a_item,
            // No base item - base unloaded item with discarded absolute mutation values
            None => {
                return Self {
                    base: SolItemBase::new_with_id_unloaded(id, type_id, state),
                    mutation: Some(convert_mutation_basic(mutation_request)),
                }
            }
        };
        let a_mutator = match src.get_a_muta(&mutation_request.mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - base loaded item with discarded absolute mutation values
            None => {
                return Self {
                    base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
                    mutation: Some(convert_mutation_basic(mutation_request)),
                }
            }
        };
        let mutated_type_id = match a_mutator.item_map.get(&type_id) {
            Some(mutated_type_id) => *mutated_type_id,
            // No mutated item type ID - base item, but with all the mutations applied. Unlike on
            // previous steps, here it's possible to convert absolute mutated attribute values into
            // rolls using base item attributes as unmutated values
            None => {
                return Self {
                    base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
                    mutation: Some(convert_mutation_full(
                        mutation_request,
                        &base_a_item.attr_vals,
                        a_mutator,
                    )),
                }
            }
        };
        let mutated_a_item = match src.get_a_item(&mutated_type_id) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated item - same as previous step, i.e. base item, but with all the mutations
            // applied
            None => {
                return Self {
                    base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
                    mutation: Some(convert_mutation_full(
                        mutation_request,
                        &base_a_item.attr_vals,
                        a_mutator,
                    )),
                }
            }
        };
        // Make proper mutated item once we have all the data
        let mut attrs = merge_attrs(base_a_item, mutated_a_item);
        let mut item_mutation = convert_mutation_full(mutation_request, &attrs, a_mutator);
        apply_attr_mutations(&mut attrs, a_mutator, &item_mutation.attr_rolls);
        item_mutation.cache = Some(SolItemMutationDataCache::new(type_id, attrs));
        Self {
            base: SolItemBase::new_with_item(id, base_a_item.clone(), state),
            mutation: Some(item_mutation),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol::item) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol::item) fn get_group_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.base.get_group_id()
    }
    pub(in crate::sol::item) fn get_category_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.base.get_category_id()
    }
    pub(in crate::sol::item) fn get_attrs(&self) -> Result<&StMap<EAttrId, AttrVal>, ItemLoadedError> {
        let item_mutation = match &self.mutation {
            Some(item_mutation) => item_mutation,
            None => return self.base.get_attrs(),
        };
        match &item_mutation.cache {
            Some(cache) => Ok(&cache.merged_attrs),
            None => self.base.get_attrs(),
        }
    }
    pub(in crate::sol::item) fn get_effect_datas(
        &self,
    ) -> Result<&StMap<EEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.base.get_effect_datas()
    }
    pub(in crate::sol::item) fn get_defeff_id(&self) -> Result<Option<EEffectId>, ItemLoadedError> {
        self.base.get_defeff_id()
    }
    pub(in crate::sol::item) fn get_skill_reqs(&self) -> Result<&StMap<EItemId, SkillLevel>, ItemLoadedError> {
        self.base.get_skill_reqs()
    }
    pub(in crate::sol::item) fn get_state(&self) -> SolItemState {
        self.base.get_state()
    }
    pub(in crate::sol::item) fn set_state(&mut self, state: SolItemState) {
        self.base.set_state(state)
    }
    pub(in crate::sol::item) fn get_effect_modes(&self) -> &SolEffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol::item) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::item) fn update_a_data(&mut self, src: &Src) {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            // No mutation - just update base item
            None => {
                self.base.update_a_data(src);
                return;
            }
        };
        let base_type_id = match &item_mutation.cache {
            Some(cache) => cache.base_type_id,
            None => self.base.get_type_id(),
        };
        let base_a_item = match src.get_a_item(&base_type_id) {
            Some(base_a_item) => base_a_item,
            // No base item - invalidate mutated cache and use base item data we have, i.e. just ID
            None => {
                self.base.set_type_id(base_type_id);
                self.base.remove_a_item();
                item_mutation.cache = None;
                return;
            }
        };
        let a_mutator = match src.get_a_muta(&item_mutation.mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - invalidate mutated cache and use non-mutated item
            None => {
                self.base.set_type_id(base_type_id);
                self.base.set_a_item(base_a_item.clone());
                item_mutation.cache = None;
                return;
            }
        };
        let mutated_type_id = match a_mutator.item_map.get(&base_type_id) {
            Some(mutated_type_id) => *mutated_type_id,
            // No mutated item type ID - invalidate mutated cache and use non-mutated item
            None => {
                self.base.set_type_id(base_type_id);
                self.base.set_a_item(base_a_item.clone());
                item_mutation.cache = None;
                return;
            }
        };
        let mutated_a_item = match src.get_a_item(&mutated_type_id) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated item - invalidate mutated cache and use non-mutated item
            None => {
                self.base.set_type_id(base_type_id);
                self.base.set_a_item(base_a_item.clone());
                item_mutation.cache = None;
                return;
            }
        };
        // Compose attribute cache
        let mut attrs = merge_attrs(base_a_item, mutated_a_item);
        apply_attr_mutations(&mut attrs, a_mutator, &item_mutation.attr_rolls);
        // Everything needed is at hand, update item
        self.base.set_type_id(mutated_type_id);
        self.base.set_a_item(mutated_a_item.clone());
        item_mutation.cache = Some(SolItemMutationDataCache::new(base_type_id, attrs))
    }
    // Mutation-specific methods
    pub(in crate::sol::item) fn mutate(
        &mut self,
        src: &Src,
        mutation_request: SolItemMutation,
    ) -> Result<(), ItemNotMutatedError> {
        if self.mutation.is_some() {
            return Err(ItemNotMutatedError::new(self.get_id()));
        };
        // Since item is not mutated, base type ID is always on non-mutated item base
        let base_type_id = self.base.get_type_id();
        let base_a_item = match self.base.get_a_item() {
            Ok(base_a_item) => base_a_item,
            // No base item - discard absolute mutation values, and store the rest w/o applying
            Err(_) => {
                self.mutation = Some(convert_mutation_basic(mutation_request));
                return Ok(());
            }
        };
        let a_mutator = match src.get_a_muta(&mutation_request.mutator_id) {
            Some(a_mutator) => a_mutator,
            // No mutator - discard absolute mutation values, and store the rest w/o applying
            None => {
                self.mutation = Some(convert_mutation_basic(mutation_request));
                return Ok(());
            }
        };
        let mutated_type_id = match a_mutator.item_map.get(&base_type_id) {
            Some(mutated_type_id) => *mutated_type_id,
            // No mutated item type ID - apply all the mutations. Unlike on previous steps, here
            // it's possible to convert absolute mutated attribute values into rolls using base item
            // attributes as unmutated values
            None => {
                self.mutation = Some(convert_mutation_full(
                    mutation_request,
                    &base_a_item.attr_vals,
                    a_mutator,
                ));
                return Ok(());
            }
        };
        let mutated_a_item = match src.get_a_item(&mutated_type_id) {
            Some(mutated_a_item) => mutated_a_item,
            // No mutated item - same as previous step, i.e. all the mutations applied on top of
            // base item attributes as unmutated values
            None => {
                self.mutation = Some(convert_mutation_full(
                    mutation_request,
                    &base_a_item.attr_vals,
                    a_mutator,
                ));
                return Ok(());
            }
        };
        // Since we have all the data now, apply mutation properly
        let mut attrs = merge_attrs(base_a_item, mutated_a_item);
        let mut item_mutation = convert_mutation_full(mutation_request, &attrs, a_mutator);
        apply_attr_mutations(&mut attrs, a_mutator, &item_mutation.attr_rolls);
        item_mutation.cache = Some(SolItemMutationDataCache::new(base_type_id, attrs));
        self.base.set_type_id(mutated_type_id);
        self.base.set_a_item(mutated_a_item.clone());
        self.mutation = Some(item_mutation);
        Ok(())
    }
    pub(in crate::sol::item) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutation_requests: StMap<EAttrId, Option<SolItemAttrMutation>>,
    ) -> Result<Vec<EAttrId>, ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            None => return Err(ItemMutatedError::new(self.get_id())),
        };
        let mutation_cache = match &mut item_mutation.cache {
            Some(cache) => cache,
            // If there is no cache - mutations are not effective. In this case we apply all the
            // changes which can be applied, and return empty list, since effectively none of item
            // attributes can change regardless of what was requested
            None => {
                change_mutation_attrs_ineffective(
                    src,
                    self.base.get_attrs().ok(),
                    item_mutation,
                    attr_mutation_requests,
                );
                return Ok(Vec::new());
            }
        };
        // All the methods which set cache guarantee that all the following entities are available
        // for the source the cache was generated with, and this method is supposed to be called
        // with the same source
        let base_a_item = src.get_a_item(&mutation_cache.base_type_id).unwrap();
        let a_mutator = src.get_a_muta(&item_mutation.mutator_id).unwrap();
        let mutated_type_id = a_mutator.item_map.get(&mutation_cache.base_type_id).unwrap();
        let mutated_a_item = src.get_a_item(&mutated_type_id).unwrap();
        // Process mutation requests, recording attributes whose values were changed for the item
        let mut changed_attrs = Vec::new();
        for (attr_id, attr_mutation_request) in attr_mutation_requests.into_iter() {
            // Get unmutated value of the attribute using the same logic as during attribute merge
            // (mutated item attribute value gets priority)
            let unmutated_value = match mutated_a_item.attr_vals.get(&attr_id) {
                Some(unmutated_value) => Some(*unmutated_value),
                None => base_a_item.attr_vals.get(&attr_id).copied(),
            };
            let new_value = match attr_mutation_request {
                // Mutation change request
                Some(attr_mutation) => {
                    // Normalize request to roll
                    let attr_roll =
                        match normalize_roll_full_with_value(&attr_id, unmutated_value, a_mutator, attr_mutation) {
                            Some(attr_roll) => attr_roll,
                            // Silently skip mutations we can't do anything with
                            None => continue,
                        };
                    // Update user-defined data
                    item_mutation.attr_rolls.insert(attr_id, attr_roll);
                    // Process source-dependent data and return new value
                    let unmutated_value = match unmutated_value {
                        Some(unmutated_value) => unmutated_value,
                        // No unmutated value now means there couldn't be any mutated value with any
                        // mutation earlier as well, thus attribute value cannot change. We already
                        // updated user data, so just go to next attribute
                        None => continue,
                    };
                    let roll_range = match a_mutator.attr_mods.get(&attr_id) {
                        Some(roll_range) => roll_range,
                        // No roll range now means there couldn't be any mutated value earlier as
                        // well, regardless of user-defined roll data, thus attribute value cannot
                        // change. We already updated user data, so just go to next attribute
                        None => continue,
                    };
                    mutate_attr_value(unmutated_value, roll_range, attr_roll)
                }
                // Mutation removal request
                None => {
                    // Update user-defined data
                    item_mutation.attr_rolls.remove(&attr_id);
                    // Update source-dependent data
                    let unmutated_value = match unmutated_value {
                        Some(unmutated_value) => unmutated_value,
                        // No unmutated value - can't do any comparisons
                        None => continue,
                    };
                    unmutated_value
                }
            };
            // Since unmutated value of the attribute is available by now, we can safely assume that
            // merged attributes have some value too (those are supposed to be built using the same
            // logic as unmutated value)
            let old_value = mutation_cache.merged_attrs.insert(attr_id, new_value).unwrap();
            if old_value != new_value {
                changed_attrs.push(attr_id);
            }
        }
        Ok(changed_attrs)
    }
    pub(in crate::sol::item) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        let item_mutation = match &mut self.mutation {
            Some(item_mutation) => item_mutation,
            None => return Err(ItemMutatedError::new(self.get_id())),
        };
        match &item_mutation.cache {
            // If cache is there, mutation is effective - item base has mutated item, and base type
            // ID is stored on cache
            Some(cache) => {
                let type_id = cache.base_type_id;
                self.base.set_type_id_and_reload(src, type_id);
                self.mutation = None;
            }
            // No cache - mutation was not effective, and base item was used already. Just unassign
            // mutation in this case
            None => self.mutation = None,
        };
        Ok(())
    }
}

#[derive(Clone)]
struct SolItemMutationData {
    // User-defined data
    mutator_id: EMutaId,
    attr_rolls: StMap<EAttrId, MutaRoll>,
    // Source-dependent data
    cache: Option<SolItemMutationDataCache>,
}
impl SolItemMutationData {
    fn new(mutator_id: EMutaId) -> Self {
        Self {
            mutator_id,
            attr_rolls: StMap::new(),
            cache: None,
        }
    }
    fn new_with_attrs(mutator_id: EMutaId, attr_rolls: StMap<EAttrId, MutaRoll>) -> Self {
        Self {
            mutator_id,
            attr_rolls,
            cache: None,
        }
    }
}

// Container for data which is source-dependent
#[derive(Clone)]
struct SolItemMutationDataCache {
    base_type_id: EItemId,
    merged_attrs: StMap<EAttrId, AttrVal>,
}
impl SolItemMutationDataCache {
    fn new(base_type_id: EItemId, merged_attrs: StMap<EAttrId, AttrVal>) -> Self {
        Self {
            base_type_id,
            merged_attrs,
        }
    }
}

// Basic conversion
fn convert_mutation_basic(mutation_request: SolItemMutation) -> SolItemMutationData {
    SolItemMutationData::new_with_attrs(
        mutation_request.mutator_id,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|(k, v)| normalize_roll_simple(v).map(|v| (k, v)))
            .collect(),
    )
}

fn normalize_roll_simple(value: SolItemAttrMutation) -> Option<MutaRoll> {
    match value {
        SolItemAttrMutation::Roll(roll_value) => Some(roll_value),
        SolItemAttrMutation::Absolute(_) => None,
    }
}

// Full conversion
fn convert_mutation_full(
    mutation_request: SolItemMutation,
    unmutated_attrs: &StMap<EAttrId, AttrVal>,
    a_mutator: &ad::AMuta,
) -> SolItemMutationData {
    SolItemMutationData::new_with_attrs(
        mutation_request.mutator_id,
        mutation_request
            .attrs
            .into_iter()
            .filter_map(|(k, v)| normalize_roll_full_with_values(&k, unmutated_attrs, a_mutator, v).map(|v| (k, v)))
            .collect(),
    )
}

fn normalize_roll_full_with_values(
    attr_id: &EAttrId,
    unmutated_attrs: &StMap<EAttrId, AttrVal>,
    a_mutator: &ad::AMuta,
    attr_mutation: SolItemAttrMutation,
) -> Option<MutaRoll> {
    match attr_mutation {
        SolItemAttrMutation::Roll(roll) => Some(roll),
        SolItemAttrMutation::Absolute(absolute) => {
            let unmutated_value = match unmutated_attrs.get(attr_id) {
                Some(unmutated_value) => *unmutated_value,
                None => return None,
            };
            let (min_mult, max_mult) = match a_mutator.attr_mods.get(attr_id) {
                Some(r) => (r.min_mult, r.max_mult),
                None => return None,
            };
            let min_value = unmutated_value * min_mult;
            let max_value = unmutated_value * max_mult;
            Some((absolute - min_value) / (max_value - min_value))
        }
    }
}

fn normalize_roll_full_with_value(
    attr_id: &EAttrId,
    unmutated_value: Option<AttrVal>,
    a_mutator: &ad::AMuta,
    attr_mutation: SolItemAttrMutation,
) -> Option<MutaRoll> {
    match attr_mutation {
        SolItemAttrMutation::Roll(roll) => Some(roll),
        SolItemAttrMutation::Absolute(absolute) => {
            let unmutated_value = match unmutated_value {
                Some(unmutated_value) => unmutated_value,
                None => return None,
            };
            let (min_mult, max_mult) = match a_mutator.attr_mods.get(attr_id) {
                Some(r) => (r.min_mult, r.max_mult),
                None => return None,
            };
            let min_value = unmutated_value * min_mult;
            let max_value = unmutated_value * max_mult;
            Some((absolute - min_value) / (max_value - min_value))
        }
    }
}

// Attribute mutations
fn apply_attr_mutations(
    attrs: &mut StMap<EAttrId, AttrVal>,
    a_mutator: &ad::AMuta,
    attr_rolls: &StMap<EAttrId, MutaRoll>,
) {
    for (attr_id, attr_roll) in attr_rolls.iter() {
        let unmutated_value = match attrs.get(&attr_id) {
            Some(unmutated_value) => *unmutated_value,
            None => continue,
        };
        if let Some(roll_range) = a_mutator.attr_mods.get(&attr_id) {
            let mutated_val = mutate_attr_value(unmutated_value, roll_range, *attr_roll);
            attrs.insert(*attr_id, mutated_val);
        }
    }
}

fn mutate_attr_value(unmutated_value: AttrVal, roll_range: &ad::AMutaAttrRange, roll: MutaRoll) -> AttrVal {
    unmutated_value * (roll_range.min_mult + roll * (roll_range.max_mult - roll_range.min_mult))
}

// Misc functions
fn merge_attrs(base_a_item: &ad::AItem, mutated_a_item: &ad::AItem) -> StMap<EAttrId, AttrVal> {
    let mut attrs = base_a_item.attr_vals.clone();
    // Mutated item attributes have priority in case of collisions
    for (attr_id, attr_val) in mutated_a_item.attr_vals.iter() {
        attrs.insert(*attr_id, *attr_val);
    }
    attrs
}

fn change_mutation_attrs_ineffective(
    src: &Src,
    base_attrs: Option<&StMap<EAttrId, AttrVal>>,
    item_mutation: &mut SolItemMutationData,
    attr_mutation_requests: StMap<EAttrId, Option<SolItemAttrMutation>>,
) {
    match (base_attrs, src.get_a_muta(&item_mutation.mutator_id)) {
        // Cache might've been not set due to a number of reasons. In case mutated item was not
        // available but mutator was, and we have access to base item attributes - we can
        // extract more data from mutation request (by converting absolute values into roll
        // ranges relatively base item attributes)
        (Some(base_item_attrs), Some(a_mutator)) => {
            for (attr_id, attr_mutation_request) in attr_mutation_requests.into_iter() {
                match attr_mutation_request {
                    Some(val) => {
                        if let Some(val) = normalize_roll_full_with_values(&attr_id, base_item_attrs, a_mutator, val) {
                            item_mutation.attr_rolls.insert(attr_id, val);
                        }
                    }
                    None => {
                        item_mutation.attr_rolls.remove(&attr_id);
                    }
                }
            }
        }
        // When no extra info is available, we can process only basic requests, i.e. mutation
        // removal and mutation changes with roll values
        _ => {
            for (attr_id, attr_mutation_request) in attr_mutation_requests.into_iter() {
                match attr_mutation_request {
                    Some(val) => {
                        if let Some(val) = normalize_roll_simple(val) {
                            item_mutation.attr_rolls.insert(attr_id, val);
                        }
                    }
                    None => {
                        item_mutation.attr_rolls.remove(&attr_id);
                    }
                }
            }
        }
    }
}
