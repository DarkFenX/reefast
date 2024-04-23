from pytest import approx


def test_self_state_switch(client, consts):
    # Check that fleet effects are applied/removed when module carrying them changes state
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_self_buff_switch_no_default(client, consts):
    # Check that when buff reference changes, buff gets updated as well
    eve_buff_val_mult_attr = client.mk_eve_attr()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id, def_val=0)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr1 = client.mk_eve_attr()
    eve_tgt_attr2 = client.mk_eve_attr()
    eve_buff1 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr1.id)])
    eve_buff2 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr2.id)])
    eve_module_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_val_attr.id: 1.25},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_charge_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_assign,
        src_attr_id=eve_buff_type_attr.id,
        tgt_attr_id=eve_buff_type_attr.id)
    eve_charge_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_buff_val_mult_attr.id,
        tgt_attr_id=eve_buff_val_attr.id)
    eve_charge_effect = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge1 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff1.id, eve_buff_val_mult_attr.id: 4},
        eff_ids=[eve_charge_effect.id])
    eve_charge2 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff2.id, eve_buff_val_mult_attr.id: 8},
        eff_ids=[eve_charge_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr1.id: 20, eve_tgt_attr2.id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    # Action
    api_module.change_mod(charge=eve_charge1.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(100)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    # Action
    api_module.change_mod(charge=eve_charge2.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(500)
    # Action
    api_module.change_mod(charge=None)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)


def test_self_buff_switch_with_default(client, consts):
    # Check that when buff reference changes, buff gets updated as well
    eve_buff_val_mult_attr = client.mk_eve_attr()
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr1 = client.mk_eve_attr()
    eve_tgt_attr2 = client.mk_eve_attr()
    eve_tgt_attr3 = client.mk_eve_attr()
    eve_buff1 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr1.id)])
    eve_buff2 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr2.id)])
    eve_buff3 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr3.id)])
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id, def_val=eve_buff3.id)
    eve_module_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_val_attr.id: 1.25},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_charge_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_assign,
        src_attr_id=eve_buff_type_attr.id,
        tgt_attr_id=eve_buff_type_attr.id)
    eve_charge_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_buff_val_mult_attr.id,
        tgt_attr_id=eve_buff_val_attr.id)
    eve_charge_effect = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge1 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff1.id, eve_buff_val_mult_attr.id: 4},
        eff_ids=[eve_charge_effect.id])
    eve_charge2 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff2.id, eve_buff_val_mult_attr.id: 8},
        eff_ids=[eve_charge_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr1.id: 20, eve_tgt_attr2.id: 50, eve_tgt_attr3.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    assert api_ship.attrs[eve_tgt_attr3.id].dogma == approx(125)
    # Action
    api_module.change_mod(charge=eve_charge1.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(100)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    assert api_ship.attrs[eve_tgt_attr3.id].dogma == approx(100)
    # Action
    api_module.change_mod(charge=eve_charge2.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(500)
    assert api_ship.attrs[eve_tgt_attr3.id].dogma == approx(100)
    # Action
    api_module.change_mod(charge=None)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    assert api_ship.attrs[eve_tgt_attr3.id].dogma == approx(125)


def test_self_after_fleet_unassigment(client, consts):
    # Check that fleet effects stay even after a fit has been removed from a fleet
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit()
    api_fit.set_fleet(fleet_id=api_fleet.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fit.set_fleet(fleet_id=None)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)


def test_self_after_fleet_removal(client, consts):
    # Check that fleet effects stay even after the fleet has been removed
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit()
    api_fit.set_fleet(fleet_id=api_fleet.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fleet.remove()
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)


def test_fleeted_state_switch(client, consts):
    # Check that fleet effects are applied/removed when module carrying them changes state
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_fleeted_buff_switch(client, consts):
    # Check that when buff reference changes, buff gets updated as well
    eve_buff_val_mult_attr = client.mk_eve_attr()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id, def_val=0)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr1 = client.mk_eve_attr()
    eve_tgt_attr2 = client.mk_eve_attr()
    eve_buff1 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr1.id)])
    eve_buff2 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr2.id)])
    eve_module_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_val_attr.id: 1.25},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_charge_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_assign,
        src_attr_id=eve_buff_type_attr.id,
        tgt_attr_id=eve_buff_type_attr.id)
    eve_charge_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_buff_val_mult_attr.id,
        tgt_attr_id=eve_buff_val_attr.id)
    eve_charge_effect = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge1 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff1.id, eve_buff_val_mult_attr.id: 4},
        eff_ids=[eve_charge_effect.id])
    eve_charge2 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff2.id, eve_buff_val_mult_attr.id: 8},
        eff_ids=[eve_charge_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr1.id: 20, eve_tgt_attr2.id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    # Action
    api_module.change_mod(charge=eve_charge1.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(100)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)
    # Action
    api_module.change_mod(charge=eve_charge2.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(500)
    # Action
    api_module.change_mod(charge=None)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr1.id].dogma == approx(20)
    assert api_ship.attrs[eve_tgt_attr2.id].dogma == approx(50)


def test_fleeted_booster_added_removed(client, consts):
    # Check that fleet effects are applied/removed when boosting fit joins/leaves
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit2.set_fleet(fleet_id=api_fleet.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fit1.set_fleet(fleet_id=api_fleet.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fit1.set_fleet(fleet_id=None)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_fleeted_boosted_fit_added_removed(client, consts):
    # Check that fleet effects are applied/removed when boosted fit joins/leaves
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit1.set_fleet(fleet_id=api_fleet.id)
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fit2.set_fleet(fleet_id=api_fleet.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fit2.set_fleet(fleet_id=None)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_fleeted_boosted_item_added_removed(client, consts):
    # Check that fleet effects are applied through boosted item removal and addition
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    eve_ship2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_ship1 = api_fit2.set_ship(type_id=eve_ship1.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(75)


def test_fleeted_fleet_removed(client, consts):
    # Check that fleet effects are removed when fleet is removed
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fleet.remove()
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_on_fleet_add(client, consts):
    # Fleet effects shouldn't apply outside carrying fit and its fleet regardless of circumstances
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship.id)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fleet.change(add_fits=[api_fit1.id])
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fleet.change(remove_fits=[api_fit1.id])
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
