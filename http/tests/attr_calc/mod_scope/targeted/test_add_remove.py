from pytest import approx


def test_tgt_untgt(client, consts):
    # Check that effects are applied/removed when module is targeted/untargeted
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_module.api_module(rm_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
