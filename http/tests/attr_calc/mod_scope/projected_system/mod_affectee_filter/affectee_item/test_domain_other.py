from pytest import approx


def test_unaffected_self(client, consts):
    # Check that other reference from projected effect doesn't affect anything
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.create_fit()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    assert api_proj_effect.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_root(client, consts):
    # Check that other reference from projected effect doesn't affect anything
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that other reference from projected effect doesn't affect anything
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(100)
