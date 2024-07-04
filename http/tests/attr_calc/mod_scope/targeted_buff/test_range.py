from pytest import approx


def test_optimal_unavailable(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_optimal_attr.id)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, None)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 0)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 1)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)
