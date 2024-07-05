from pytest import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_mult_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_module_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 50},
        eff_ids=[eve_module_effect.id], defeff_id=eve_module_effect.id)
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_buff_mult_attr.id,
        affectee_attr_id=eve_buff_val_attr.id)
    eve_implant_effect = client.mk_eve_effect(mod_info=[eve_implant_mod])
    eve_implant = client.mk_eve_item(attrs={eve_buff_mult_attr.id: 2}, eff_ids=[eve_implant_effect.id])
    eve_ship1 = client.mk_eve_ship()
    eve_ship2 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.set_ship(type_id=eve_ship1.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(11.25)
    api_implant = api_fit1.add_implant(type_id=eve_implant.id)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(15)
    api_implant.remove()
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(11.25)


def test_self_replace_root(client, consts):
    # Make sure ship is affected by fleet buffs even if it was replaced
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    eve_ship2 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship1.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_ship2 = api_fit.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(75)


def test_fleeted_replace_root(client, consts):
    # Make sure ship is affected by fleet buffs even if it was replaced
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    eve_ship2 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_ship1 = api_fit2.set_ship(type_id=eve_ship1.id)
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(75)
