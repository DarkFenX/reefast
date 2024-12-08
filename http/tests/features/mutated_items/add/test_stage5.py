"""
Stage 5 means that all the prerequisites for mutated item have been met:
- base item is available;
- mutator is available;
- mutated item ID and item itself are available.
"""

from tests import approx, check_no_field


def test_rolls_range(client, consts):
    # Check processing of roll values - within range and out of range
    eve_lower_attr = client.mk_eve_attr()
    eve_within_attr = client.mk_eve_attr()
    eve_higher_attr = client.mk_eve_attr()
    eve_base_item = client.mk_eve_item(attrs={eve_lower_attr.id: 100, eve_within_attr.id: 100, eve_higher_attr.id: 100})
    eve_mutated_item = client.mk_eve_item()
    eve_mutator = client.mk_eve_mutator(
        items=[([eve_base_item.id], eve_mutated_item.id)],
        attributes={eve_lower_attr.id: (0.8, 1.2), eve_within_attr.id: (0.8, 1.2), eve_higher_attr.id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item.id, mutation=(eve_mutator.id, {
        eve_lower_attr.id: {consts.ApiAttrMutation.roll: -5},
        eve_within_attr.id: {consts.ApiAttrMutation.roll: 0.3},
        eve_higher_attr.id: {consts.ApiAttrMutation.roll: 128}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_lower_attr.id].roll == approx(0)
    assert api_item.mutation.attrs[eve_lower_attr.id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_within_attr.id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_within_attr.id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_higher_attr.id].roll == approx(1)
    assert api_item.mutation.attrs[eve_higher_attr.id].absolute == approx(120)
    assert api_item.attrs[eve_lower_attr.id].base == approx(80)
    assert api_item.attrs[eve_within_attr.id].base == approx(92)
    assert api_item.attrs[eve_higher_attr.id].base == approx(120)


def test_absolute_base_attr_value(client, consts):
    # Check what is used as base attribute value for converting absolute value into roll
    eve_base_attr = client.mk_eve_attr()
    eve_overlap_attr = client.mk_eve_attr()
    eve_mutated_attr = client.mk_eve_attr()
    eve_base_item = client.mk_eve_item(attrs={eve_base_attr.id: 50, eve_overlap_attr.id: 70})
    eve_mutated_item = client.mk_eve_item(attrs={eve_overlap_attr.id: 80, eve_mutated_attr.id: 100})
    eve_mutator = client.mk_eve_mutator(
        items=[([eve_base_item.id], eve_mutated_item.id)],
        attributes={eve_base_attr.id: (0.8, 1.2), eve_overlap_attr.id: (0.8, 1.2), eve_mutated_attr.id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item.id, mutation=(eve_mutator.id, {
        eve_base_attr.id: {consts.ApiAttrMutation.absolute: 55},
        eve_overlap_attr.id: {consts.ApiAttrMutation.absolute: 75},
        eve_mutated_attr.id: {consts.ApiAttrMutation.absolute: 115}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_base_attr.id].roll == approx(0.75)
    assert api_item.mutation.attrs[eve_base_attr.id].absolute == approx(55)
    # For overlapping values, mutated item values should be taken; we check it here via roll value,
    # which is below 0.5 if base value is 80 instead of 70
    assert api_item.mutation.attrs[eve_overlap_attr.id].roll == approx(0.34375)
    assert api_item.mutation.attrs[eve_overlap_attr.id].absolute == approx(75)
    assert api_item.mutation.attrs[eve_mutated_attr.id].roll == approx(0.875)
    assert api_item.mutation.attrs[eve_mutated_attr.id].absolute == approx(115)
    assert api_item.attrs[eve_base_attr.id].base == approx(55)
    assert api_item.attrs[eve_overlap_attr.id].base == approx(75)
    assert api_item.attrs[eve_mutated_attr.id].base == approx(115)


def test_absolute_range(client, consts):
    # Check processing of absolute values - within range and out of range
    eve_lower_attr = client.mk_eve_attr()
    eve_within_attr = client.mk_eve_attr()
    eve_higher_attr = client.mk_eve_attr()
    eve_base_item = client.mk_eve_item(attrs={eve_lower_attr.id: 100, eve_within_attr.id: 100, eve_higher_attr.id: 100})
    eve_mutated_item = client.mk_eve_item()
    eve_mutator = client.mk_eve_mutator(
        items=[([eve_base_item.id], eve_mutated_item.id)],
        attributes={eve_lower_attr.id: (0.8, 1.2), eve_within_attr.id: (0.8, 1.2), eve_higher_attr.id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item.id, mutation=(eve_mutator.id, {
        eve_lower_attr.id: {consts.ApiAttrMutation.absolute: -53},
        eve_within_attr.id: {consts.ApiAttrMutation.absolute: 92},
        eve_higher_attr.id: {consts.ApiAttrMutation.absolute: 1009}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_lower_attr.id].roll == approx(0)
    assert api_item.mutation.attrs[eve_lower_attr.id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_within_attr.id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_within_attr.id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_higher_attr.id].roll == approx(1)
    assert api_item.mutation.attrs[eve_higher_attr.id].absolute == approx(120)
    assert api_item.attrs[eve_lower_attr.id].base == approx(80)
    assert api_item.attrs[eve_within_attr.id].base == approx(92)
    assert api_item.attrs[eve_higher_attr.id].base == approx(120)


def test_no_base_value(client, consts):
    # Rolls accepted, absolutes discarded
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_roll_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_roll_attr_id)
    eve_absolute_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_absolute_attr_id)
    eve_base_item_id = eve_d1.mk_item().id
    eve_d2.mk_item(id_=eve_base_item_id, attrs={eve_roll_attr_id: 50, eve_absolute_attr_id: 50})
    eve_mutated_item_id = eve_d1.mk_item().id
    eve_d2.mk_item(id_=eve_mutated_item_id)
    eve_mutator_id = eve_d1.mk_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)}).id
    eve_d2.mk_mutator(
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_roll_attr_id: {consts.ApiAttrMutation.roll: 0.7},
        eve_absolute_attr_id: {consts.ApiAttrMutation.absolute: 54}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 0
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    # Since there were no base attribute values on first source, attribute mutations defined via
    # absolute value are discarded. However, on second source roll and absolute value are still
    # exposed, but not for absolute value we passed earlier, but for base attribute value
    api_sol.change_src(data=eve_d2)
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.7)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(54)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(50)
    assert api_item.attrs[eve_roll_attr_id].base == approx(54)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(50)


def test_modification(client, consts):
    # Check that mutated value is used as base for source and target of modifications
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_base_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 200})
    eve_mutated_item = client.mk_eve_item(eff_ids=[eve_effect.id])
    eve_mutator = client.mk_eve_mutator(
        items=[([eve_base_item.id], eve_mutated_item.id)],
        attributes={eve_affector_attr.id: (0.8, 1.2), eve_affectee_attr.id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item.id, mutation=(eve_mutator.id, {
        eve_affector_attr.id: {consts.ApiAttrMutation.roll: 0.2},
        eve_affectee_attr.id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_affector_attr.id].roll == approx(0.2)
    assert api_item.mutation.attrs[eve_affector_attr.id].absolute == approx(17.6)
    assert api_item.mutation.attrs[eve_affectee_attr.id].roll == approx(0.8)
    assert api_item.mutation.attrs[eve_affectee_attr.id].absolute == approx(224)
    assert api_item.attrs[eve_affector_attr.id].base == approx(17.6)
    assert api_item.attrs[eve_affector_attr.id].dogma == approx(17.6)
    assert api_item.attrs[eve_affectee_attr.id].base == approx(224)
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(263.424)
