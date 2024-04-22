from pytest import approx


def test_affected_self_child_ship(client, consts):
    # Make sure ship items (such as rigs) are affected by fleet buffs
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)


def test_affected_fleeted_child_ship(client, consts):
    # Make sure ship items (such as rigs) are affected by fleet buffs
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fleet = api_ss.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit2.set_ship(type_id=eve_ship.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)


def test_affected_change_chain(client, consts):
    # Check that when buff value is changed, affectee attribute value is updated
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_mult_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 50},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_buff_mult_attr.id,
        tgt_attr_id=eve_buff_val_attr.id)
    eve_implant_effect = client.mk_eve_effect(mod_info=[eve_implant_mod])
    eve_implant = client.mk_eve_item(attrs={eve_buff_mult_attr.id: 2}, eff_ids=[eve_implant_effect.id])
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fleet = api_ss.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.set_ship(type_id=eve_ship.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(11.25)
    api_implant = api_fit1.add_implant(type_id=eve_implant.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(15)
    api_implant.remove()
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(11.25)


def test_unaffected_other_group(client, consts):
    # Check that entities belonging to other item groups are not affected
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp1.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_self_child_struct(client, consts):
    # Make sure structures are not affected by fleet buffs
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_struct(type_id=eve_struct.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_fleeted_child_struct(client, consts):
    # Make sure structures are not affected by fleet buffs
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fleet = api_ss.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit2.set_struct(type_id=eve_struct.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_fit(client, consts):
    # Check that fits outside of fleet are not affected
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_fleet(client, consts):
    # Check that fits outside of fleet are not affected
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fleet1 = api_ss.create_fleet()
    api_fleet2 = api_ss.create_fleet()
    api_fit1 = api_ss.create_fit()
    api_fit1.set_fleet(fleet_id=api_fleet1.id)
    api_fit2 = api_ss.create_fit()
    api_fit2.set_fleet(fleet_id=api_fleet2.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_self_replace_parent(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fit.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)


def test_fleeted_replace_parent(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 7.5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fleet = api_ss.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fit2.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
