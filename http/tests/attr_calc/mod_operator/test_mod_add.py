from pytest import approx


def setup_penalization_test(client, consts, stackable):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1 = client.mk_eve_item(attrs={eve_affector_attr.id: 10}, eff_ids=[eve_effect.id])
    eve_item_affector2 = client.mk_eve_item(attrs={eve_affector_attr.id: -20}, eff_ids=[eve_effect.id])
    eve_item_affector3 = client.mk_eve_item(attrs={eve_affector_attr.id: 53.02}, eff_ids=[eve_effect.id])
    # Addition of 0 is considered insignificant, and won't be exposed as modification
    eve_item_affector4 = client.mk_eve_item(attrs={eve_affector_attr.id: 0}, eff_ids=[eve_effect.id])
    eve_item_affectee = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1.id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2.id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3.id)
    api_fit.add_rig(type_id=eve_item_affector4.id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee.id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr.id].dogma,
        api_item_affectee.mods[eve_affectee_attr.id],
        api_item_affector1,
        api_item_affector2,
        api_item_affector3)


def test_non_penalized(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2, api_item_affector3 = setup_penalization_test(
        client, consts, stackable=True)
    assert attr_val == approx(143.02)
    assert len(attr_mods) == 3
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.val == approx(10)
    assert api_mod1.op == consts.ApiModOp.mod_add
    assert api_mod1.penalized is False
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.val == approx(-20)
    assert api_mod2.op == consts.ApiModOp.mod_add
    assert api_mod2.penalized is False
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.val == approx(53.02)
    assert api_mod3.op == consts.ApiModOp.mod_add
    assert api_mod3.penalized is False


def test_penalized(client, consts):
    attr_val, attr_mods, api_item_affector1, api_item_affector2, api_item_affector3 = setup_penalization_test(
        client, consts, stackable=False)
    assert attr_val == approx(143.02)
    assert len(attr_mods) == 3
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.val == approx(10)
    assert api_mod1.op == consts.ApiModOp.mod_add
    assert api_mod1.penalized is False
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.val == approx(-20)
    assert api_mod2.op == consts.ApiModOp.mod_add
    assert api_mod2.penalized is False
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.val == approx(53.02)
    assert api_mod3.op == consts.ApiModOp.mod_add
    assert api_mod3.penalized is False
