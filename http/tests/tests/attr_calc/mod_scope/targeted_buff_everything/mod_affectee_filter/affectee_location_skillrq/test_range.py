from tests import approx


def test_range(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id)
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_module(add_projs=[(api_affectee_ship.id, 9000)])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_affector_module.change_module(change_projs=[(api_affectee_ship.id, 11000)])
    # Falloff attribute is ignored for buffs
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_module(rm_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(200)
