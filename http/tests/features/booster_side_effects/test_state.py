from tests import approx


def test_booster_state(client, consts):
    eve_chance_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(chance_attr_id=eve_chance_attr.id, mod_info=[eve_mod])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 200})
    eve_booster = client.mk_eve_item(
        attrs={eve_chance_attr.id: 0.4, eve_affector_attr.id: 25},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_booster = api_fit.add_booster(type_id=eve_booster.id, side_effects={eve_effect.id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr.id].extra == approx(250)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(state=False)
    # Verification
    # Side effect modification is disabled, because parent booster is disabled
    assert api_ship.update().attrs[eve_affectee_attr.id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect.id]
    assert api_side.chance == approx(0.4)
    # Side effect status is reported regardless of if effect is running or not
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
