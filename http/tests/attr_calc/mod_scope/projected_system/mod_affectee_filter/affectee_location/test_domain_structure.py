from pytest import approx


def test_affected_state_change(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id, state=False)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(state=True)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_proj_effect.change_proj_effect(state=False)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_affected_targeting(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_proj_effect.change_proj_effect(rm_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_domain(client, consts):
    # Check that entities from other domains are not affected
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that projected modifications are not carried over to another fit
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct1 = api_fit1.set_ship(type_id=eve_struct.id)
    api_fit2.set_ship(type_id=eve_struct.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_struct1.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_replace_root(client, consts):
    # Check structure replacement process in context of projected effect. This is not usual
    # replacement test since projection is dropped as soon as we replace structure
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_struct.remove()
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(120)
