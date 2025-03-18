from tests import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(
        attrs={eve_middle_attr_id: 20},
        eff_ids=[eve_middle_effect_id],
        defeff_id=eve_middle_effect_id)
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 80}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    eve_char_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_id)
    api_fit2.set_char(type_id=eve_char_item_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit2.add_drone(type_id=eve_affectee_item_id)
    api_middle_item = api_fit1.add_module(type_id=eve_middle_item_id, state=consts.ApiModuleState.active)
    api_middle_item.change_module(add_projs=[api_ship.id])
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    api_affector_item = api_fit1.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(104)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(96)


def test_replace_root(client, consts):
    # This behavior isn't defined in EVE, but we check how character presence influences
    # modifications with owner-skillreq filter. In our case it doesn't, because those are tracked
    # by fit ID
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_char = api_fit2.set_char(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)
    api_char.remove()
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)
    api_fit2.set_char(type_id=eve_char_id)
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)


def test_replace_proj(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_char(type_id=eve_char_id)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship1.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)
    api_ship1.remove()
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship_id)
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)
    api_module.change_module(add_projs=[api_ship2.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)
