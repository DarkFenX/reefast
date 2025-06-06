from tests import approx


def test_optimal_unavailable(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_optimal_attr_id)
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_module(add_projs=[(api_affectee_ship.id, None)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_affector_module.change_module(change_projs=[(api_affectee_ship.id, 0)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_affector_module.change_module(change_projs=[(api_affectee_ship.id, 1)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_module(rm_projs=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)
