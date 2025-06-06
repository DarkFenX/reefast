from tests import approx


def test_add_afor_afee_proj_remove_state_proj_fit(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 80}, srqs={eve_skill_id: 1})
    eve_affectee_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct_id)
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_module.change_module(add_projs=[api_affectee_struct.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    api_affector_module.change_module(state=consts.ApiModuleState.online)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_module.change_module(rm_projs=[api_affectee_struct.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affectee_fit.remove()
    api_affectee_drone.update(status_code=404)
    api_affector_fit.remove()


def test_add_afee_afor_proj_state_remove_afor_afee(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 80}, srqs={eve_skill_id: 1})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.online)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_module.change_module(state=consts.ApiModuleState.active)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    api_affector_module.remove()
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affectee_drone.remove()
    api_affectee_drone.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_add_afee_afor_proj_remove_afee(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 80}, srqs={eve_skill_id: 1})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    api_affectee_drone.remove()
    api_affectee_drone.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()
