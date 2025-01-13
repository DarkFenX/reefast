from tests import check_no_field


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150


def test_fail_multiple(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 2
    assert api_val.details.calibration.users[api_rig1.id] == 50
    assert api_val.details.calibration.users[api_rig2.id] == 100


def test_modified_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_online_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 100},eff_ids=[eve_online_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50},eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150


def test_modified_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_online_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150},eff_ids=[eve_online_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 200})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50},eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 100
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150


def test_sum_rounding(client, consts):
    # Check that total sum of users is rounded; if there would be no rounding, one of sums of 0.1
    # elements would lead to float inaccuracies
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.1}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0.15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    for i in range(1, 21):
        # Action
        api_fit.add_rig(type_id=eve_rig_id)
        if i == 1:
            continue
        # Verification
        api_val = api_fit.validate(include=[consts.ApiValType.calibration])
        assert api_val.passed is False
        assert api_val.details.calibration.used == round(i / 10, 1)
        assert api_val.details.calibration.output == 0.15
        assert len(api_val.details.calibration.users) == i


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 5
    assert api_val.details.calibration.output is None
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 5


def test_unloaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 5
    assert api_val.details.calibration.output is None
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 5


def test_unloaded_user(client, consts):
    # Just check that nothing crashes, unloaded items are not supposed to even be registered
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_rig_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_rig3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    api_fit.add_rig(type_id=eve_rig3_id)
    # Verification - items with negative and 0 use are not exposed
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 140
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig2.id] == 150


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.upgrade_cost
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True


def test_no_attr_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = consts.EveAttr.upgrade_capacity
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output is None
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150


def test_criterion_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_rig.change_rig(state=True)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150
    # Action
    api_rig.change_rig(state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_effect(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150
    # Action
    api_rig.change_rig(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_rig.change_rig(state=True, effect_modes={eve_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == 150
    assert api_val.details.calibration.output == 125
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == 150
    # Action
    api_rig.change_rig(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_rig(client, consts):
    # Validation applies only to rigs
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_rig_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
