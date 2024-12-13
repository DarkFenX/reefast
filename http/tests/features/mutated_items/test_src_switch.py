from tests import approx


def test_complete_to_complete_different_base_value(client, consts):
    # Check how mutation values are transferred upon new base attribute value on new source
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_base_item_id,
        attrs={eve_lower_attr_id: 100, eve_within_attr_id: 100, eve_higher_attr_id: 100})
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_base_item_id,
        attrs={eve_lower_attr_id: 80, eve_within_attr_id: 100, eve_higher_attr_id: 120})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.mk_eve_mutator(
        datas=[eve_d1, eve_d2],
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_lower_attr_id: {consts.ApiAttrMutation.absolute: 80},
        eve_within_attr_id: {consts.ApiAttrMutation.absolute: 100},
        eve_higher_attr_id: {consts.ApiAttrMutation.absolute: 120}}))
    # Verification
    api_item.update()
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(80)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(120)
    assert api_item.attrs[eve_lower_attr_id].base == approx(80)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(64)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(144)
    assert api_item.attrs[eve_lower_attr_id].base == approx(64)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(144)


def test_complete_to_complete_different_ranges(client, consts):
    # Check how mutation values are transferred upon new mutation ranges on new source
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_lower_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_within_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_higher_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_lower_attr_id: 100, eve_within_attr_id: 100, eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(
        datas=[eve_d1],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_lower_attr_id: (0.8, 1.2), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.8, 1.2)})
    client.mk_eve_mutator(
        datas=[eve_d2],
        id_=eve_mutator_id,
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attributes={eve_lower_attr_id: (0.7, 1.1), eve_within_attr_id: (0.8, 1.2), eve_higher_attr_id: (0.9, 1.3)})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=(eve_mutator_id, {
        eve_lower_attr_id: {consts.ApiAttrMutation.absolute: 100},
        eve_within_attr_id: {consts.ApiAttrMutation.absolute: 100},
        eve_higher_attr_id: {consts.ApiAttrMutation.absolute: 100}}))
    # Verification
    api_item.update()
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_lower_attr_id].base == approx(100)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert api_item.mutation.attrs[eve_lower_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_lower_attr_id].absolute == approx(90)
    assert api_item.mutation.attrs[eve_within_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_within_attr_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_higher_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_higher_attr_id].absolute == approx(110)
    assert api_item.attrs[eve_lower_attr_id].base == approx(90)
    assert api_item.attrs[eve_within_attr_id].base == approx(100)
    assert api_item.attrs[eve_higher_attr_id].base == approx(110)


def test_complete_to_complete_same_id(client, consts):
    # Check that new mutated item is used on new source, even if ID is the same
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_grp1_id = client.mk_eve_item_group(datas=[eve_d1, eve_d2])
    eve_grp2_id = client.mk_eve_item_group(datas=[eve_d1, eve_d2])
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affectee_attr1_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affectee_attr2_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d1, eve_d2], mod_info=[eve_mod1, eve_mod2])
    eve_implant_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id])
    eve_base_item_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100})
    eve_mutated_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_mutated_item_id, grp_id=eve_grp1_id)
    client.mk_eve_item(datas=[eve_d2], id_=eve_mutated_item_id, grp_id=eve_grp2_id)
    eve_mutator_id = client.mk_eve_mutator(datas=[eve_d1, eve_d2], items=[([eve_base_item_id], eve_mutated_item_id)])
    eve_ship_id = client.mk_eve_ship(datas=[eve_d1, eve_d2])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(120)
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - by checking that different attribute is affected, we check that new group is
    # used
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(100)
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(120)


def test_complete_to_complete_different_id(client, consts):
    # Check that mutated item is defined by base item ID and mutator ID, in this case it should
    # be different on second source
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    eve_d1_mutated_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_d1_mutated_item_id)
    eve_d2_mutated_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d2], id_=eve_d2_mutated_item_id)
    eve_mutator_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_mutator(datas=[eve_d1], id_=eve_mutator_id, items=[([eve_base_item_id], eve_d1_mutated_item_id)])
    client.mk_eve_mutator(datas=[eve_d2], id_=eve_mutator_id, items=[([eve_base_item_id], eve_d2_mutated_item_id)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_d1_mutated_item_id
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_d2_mutated_item_id
