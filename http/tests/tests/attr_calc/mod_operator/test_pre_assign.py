from tests import approx


def setup_hig_test(*, client, consts, high_is_good):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(high_is_good=high_is_good)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: -20}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 53.02}, eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector2,
        api_item_affector3)


def test_high_is_good(client, consts):
    attr_val, attr_mods, _, api_item_affector3 = setup_hig_test(client=client, consts=consts, high_is_good=True)
    # Verification
    assert attr_val == approx(53.02)
    attr_mod = attr_mods.one()
    assert attr_mod.op == consts.ApiModOp.pre_assign
    assert attr_mod.initial_val == approx(53.02)
    assert attr_mod.stacking_mult is None
    assert attr_mod.applied_val == approx(53.02)
    assert attr_mod.affectors.one().item_id == api_item_affector3.id


def test_high_is_bad(client, consts):
    attr_val, attr_mods, api_item_affector2, _ = setup_hig_test(client=client, consts=consts, high_is_good=False)
    # Verification
    assert attr_val == approx(-20)
    attr_mod = attr_mods.one()
    assert attr_mod.op == consts.ApiModOp.pre_assign
    assert attr_mod.initial_val == approx(-20)
    assert attr_mod.stacking_mult is None
    assert attr_mod.applied_val == approx(-20)
    assert attr_mod.affectors.one().item_id == api_item_affector2.id
