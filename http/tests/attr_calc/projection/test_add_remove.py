from pytest import approx


def test_add_afee_afor_proj_remove_proj_afee_afor(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_affector_module.change_mod(add_tgts=[(api_affectee_ship.id, 15000)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_affector_module.change_mod(rm_tgts=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_affectee_ship.remove()
    api_affectee_ship.update(status_code=404)
    api_affector_module.remove()
    api_affectee_fit.remove()
    api_affector_fit.remove()
