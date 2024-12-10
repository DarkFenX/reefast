from tests import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_affector_effect_id])
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_middle_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_middle_item_id = client.mk_eve_item(
        attrs={eve_middle_attr_id: -55},
        eff_ids=[eve_middle_effect_id],
        defeff_id=eve_middle_effect_id)
    eve_affectee_item_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 200})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit2.add_rig(type_id=eve_affectee_item_id)
    api_middle_item = api_fit1.add_mod(type_id=eve_middle_item_id, state=consts.ApiState.active)
    api_middle_item.change_mod(add_projs=[api_ship.id])
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_affector_item = api_fit1.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(35)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(90)


def test_replace_proj_ship_to_ship(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 200})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_module = api_affector_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_ship1 = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_affectee_fit.add_rig(type_id=eve_rig_id)
    api_module.change_mod(add_projs=[api_ship1.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_ship2 = api_affectee_fit.set_ship(type_id=eve_ship_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)


def test_replace_proj_ship_to_struct(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 200})
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_module = api_affector_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_affectee_fit.add_rig(type_id=eve_rig_id)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_ship.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_struct = api_affectee_fit.set_ship(type_id=eve_struct_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.change_mod(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
