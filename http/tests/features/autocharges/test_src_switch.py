from tests import approx, check_no_field


def test_specified_same(client, consts):
    # Autocharges which have the same type ID
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_autocharge_attr_id = client.mk_eve_attr(
        datas=[eve_d1, eve_d2],
        id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_d1_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_attr_id)
    eve_d2_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_attr_id)
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_autocharge_id, attrs={eve_d1_attr_id: 50})
    client.mk_eve_item(datas=[eve_d2], id_=eve_autocharge_id, attrs={eve_d2_attr_id: 70})
    eve_fighter_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_autocharge_id
    assert api_autocharge.attrs[eve_d1_attr_id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_autocharge_id
    assert api_autocharge.attrs[eve_d2_attr_id].dogma == approx(70)


def test_specified_different(client, consts):
    # Autocharges which have different type IDs
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_autocharge_attr_id = client.mk_eve_attr(
        datas=[eve_d1, eve_d2],
        id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_d1_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_attr_id)
    eve_d2_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_attr_id)
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_d1_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_d1_autocharge_id, attrs={eve_d1_attr_id: 50})
    eve_d2_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d2], id_=eve_d2_autocharge_id, attrs={eve_d2_attr_id: 70})
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d1_autocharge_id},
        eff_ids=[eve_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d2_autocharge_id},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_d1_autocharge_id
    assert api_autocharge.attrs[eve_d1_attr_id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_d2_autocharge_id
    assert api_autocharge.attrs[eve_d2_attr_id].dogma == approx(70)


def test_valid_to_invalid_reference_to_valid(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_autocharge_attr_id = client.mk_eve_attr(
        datas=[eve_d1, eve_d2],
        id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_d1_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_attr_id)
    eve_d2_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_attr_id)
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_d1d2_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_d1d2_autocharge_id, attrs={eve_d1_attr_id: 50})
    client.mk_eve_item(datas=[eve_d2], id_=eve_d1d2_autocharge_id, attrs={eve_d2_attr_id: 70})
    # Just allocate ID, but do not create item
    eve_d2_autocharge_id = client.alloc_item_id(datas=[eve_d2])
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d1d2_autocharge_id},
        eff_ids=[eve_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d2_autocharge_id},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_d1d2_autocharge_id
    assert api_autocharge.attrs[eve_d1_attr_id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.autocharges  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_d1d2_autocharge_id
    assert api_autocharge.attrs[eve_d1_attr_id].dogma == approx(50)


def test_valid_to_no_reference_to_valid(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_autocharge_attr_id = client.mk_eve_attr(
        datas=[eve_d1, eve_d2],
        id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_d1_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_attr_id)
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_d1_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_d1_autocharge_id, attrs={eve_d1_attr_id: 50})
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d1_autocharge_id},
        eff_ids=[eve_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_fighter_id,
        eff_ids=[eve_effect_id])
    # Create item with the attribute just to make sure it is not getting removed during cleanup
    client.mk_eve_item(datas=[eve_d2], attrs={eve_autocharge_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_d1_autocharge_id
    assert api_autocharge.attrs[eve_d1_attr_id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.autocharges  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect_id]
    assert api_autocharge.type_id == eve_d1_autocharge_id
    assert api_autocharge.attrs[eve_d1_attr_id].dogma == approx(50)
