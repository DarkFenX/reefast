from tests import approx, check_no_field


def test_affected(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_character(type_id=eve_affectee_item_id)
    api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_unaffected_other_location(client, consts):
    # Make sure "top" entities described by other locations are not affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that items (in this case implant) are not affected if they belong to location even if
    # its "owner" (in this case character) is affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_implant(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that local modifications are not carried over to another fit
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.add_rig(type_id=eve_affector_item_id)
    api_affectee_item = api_fit2.set_character(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 2}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(attrs={eve_middle_attr_id: 20}, eff_ids=[eve_middle_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_character(type_id=eve_affectee_item_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_middle_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(140)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_replace_root(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_affectee_item2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item1 = api_fit.set_character(type_id=eve_affectee_item1_id)
    api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affectee_item2 = api_fit.set_character(type_id=eve_affectee_item2_id)
    assert api_affectee_item2.update().attrs[eve_affectee_attr_id].dogma == approx(60)


def setup_switch_type_id_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_loaded_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_affectee_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_affector_item_id)
    return eve_affectee_attr_id, eve_affectee_loaded_id, eve_affectee_not_loaded_id, api_fit, api_rig


def test_switch_type_id_affected_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_loaded_id,
     eve_affectee_not_loaded_id,
     api_fit,
     api_rig) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_character(type_id=eve_affectee_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_affectee_item.change_character(type_id=eve_affectee_not_loaded_id)
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018
    # Action
    api_rig.remove()
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018


def test_switch_type_id_not_loaded_to_affected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_loaded_id,
     eve_affectee_not_loaded_id,
     api_fit,
     api_rig) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_character(type_id=eve_affectee_not_loaded_id)
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018
    # Action
    api_affectee_item.change_character(type_id=eve_affectee_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_rig.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)
