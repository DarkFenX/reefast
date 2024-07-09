from pytest import approx


def setup_penalization_test(client, consts, stackable):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.2}, eff_ids=[eve_effect.id])
    eve_item_affector2 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.5}, eff_ids=[eve_effect.id])
    eve_item_affector3 = client.mk_eve_item(attrs={eve_affector_attr.id: 0.1}, eff_ids=[eve_effect.id])
    eve_item_affector4 = client.mk_eve_item(attrs={eve_affector_attr.id: 0.75}, eff_ids=[eve_effect.id])
    eve_item_affector5 = client.mk_eve_item(attrs={eve_affector_attr.id: 5}, eff_ids=[eve_effect.id])
    # Division by 1 is considered insignificant, and won't be exposed as modification
    eve_item_affector6 = client.mk_eve_item(attrs={eve_affector_attr.id: 1}, eff_ids=[eve_effect.id])
    eve_item_affectee = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1.id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2.id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3.id)
    api_item_affector4 = api_fit.add_rig(type_id=eve_item_affector4.id)
    api_item_affector5 = api_fit.add_rig(type_id=eve_item_affector5.id)
    api_fit.add_rig(type_id=eve_item_affector6.id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee.id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr.id].dogma,
        api_item_affectee.mods[eve_affectee_attr.id],
        api_item_affector1,
        api_item_affector2,
        api_item_affector3,
        api_item_affector4,
        api_item_affector5)


def test_non_penalized(client, consts):
    (attr_val,
     attr_mods,
     api_item_affector1,
     api_item_affector2,
     api_item_affector3,
     api_item_affector4,
     api_item_affector5) = setup_penalization_test(client, consts, stackable=True)
    assert attr_val == approx(148.148148)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.pre_div
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(1.2)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.pre_div
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(1.5)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.pre_div
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.pre_div
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult is None
    assert api_mod4.applied_val == approx(0.75)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.pre_div
    assert api_mod5.initial_val == approx(5)
    assert api_mod5.stacking_mult is None
    assert api_mod5.applied_val == approx(5)


def test_penalized(client, consts):
    (attr_val,
     attr_mods,
     api_item_affector1,
     api_item_affector2,
     api_item_affector3,
     api_item_affector4,
     api_item_affector5) = setup_penalization_test(client, consts, stackable=False)
    assert attr_val == approx(165.790873)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.pre_div
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.third)
    assert api_mod1.applied_val == approx(1.105091)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.pre_div
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.second)
    assert api_mod2.applied_val == approx(1.407869)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.pre_div
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.first)
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.pre_div
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.second)
    assert api_mod4.applied_val == approx(0.77537)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.pre_div
    assert api_mod5.initial_val == approx(5)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.first)
    assert api_mod5.applied_val == approx(5)


def test_zero(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector = client.mk_eve_item(attrs={eve_affector_attr.id: 0}, eff_ids=[eve_effect.id])
    eve_item_affectee = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_item_affector.id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee.id)
    # Verification
    api_item_affectee.update()
    assert api_item_affectee.attrs[eve_affectee_attr.id].dogma == approx(100)
    assert len(api_item_affectee.mods) == 0
