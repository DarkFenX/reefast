from tests import approx


def test_add_pe_item_proj_remove_state_proj_fit(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_proj_effect.change_proj_effect(state=False)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect.change_proj_effect(rm_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_fit.remove()
    api_ship.update(status_code=404)
    api_proj_effect.remove()


def test_add_item_pe_proj_state_remove_pe_item(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_char_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_char_id)
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, state=False)
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect.change_proj_effect(state=True)
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_proj_effect.remove()
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_char.remove()
    api_char.update(status_code=404)
    api_fit.remove()


def test_add_item_pe_proj_remove_item(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_struct.remove()
    api_struct.update(status_code=404)
    api_proj_effect.remove()
    api_fit.remove()
