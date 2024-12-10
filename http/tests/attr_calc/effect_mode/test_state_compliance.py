from tests import approx


def test_state_offline(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiState.offline)
    api_item.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.state_compliance})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.ghost)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance


def test_state_online(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiState.online)
    api_item.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.state_compliance})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance


def test_state_active(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiState.active)
    api_item.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.state_compliance})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance


def test_state_overload(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiState.overload)
    api_item.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.state_compliance})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
    # Action
    api_item.change_mod(state=consts.ApiState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.state_compliance
