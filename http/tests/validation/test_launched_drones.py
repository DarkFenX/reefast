from tests import check_no_field


def test_fail_single(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total == 0
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users


def test_fail_multiple_ship(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 2
    assert api_val.details.launched_drones.total == 1
    assert len(api_val.details.launched_drones.users) == 2
    assert api_drone1.id in api_val.details.launched_drones.users
    assert api_drone2.id in api_val.details.launched_drones.users


def test_fail_multiple_struct(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct(attrs={eve_output_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 2
    assert api_val.details.launched_drones.total == 1
    assert len(api_val.details.launched_drones.users) == 2
    assert api_drone1.id in api_val.details.launched_drones.users
    assert api_drone2.id in api_val.details.launched_drones.users


def test_equal(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_modified_output(client, consts):
    # Unrealistic scenario, but modification of output is supported
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total == 0
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_no_ship(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total is None
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users


def test_unloaded_ship(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total is None
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users


def test_unloaded_user(client, consts):
    # Unloaded drones still take slot
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    eve_drone_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total == 0
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users


def test_no_value_output(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    # Make an item to ensure that output attribute is not cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total == 0
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users


def test_no_attr_output(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_output_attr_id = consts.EveAttr.max_active_drones
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total is None
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users


def test_criterion_state(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_drone.change_drone(state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is False
    assert api_val.details.launched_drones.used == 1
    assert api_val.details.launched_drones.total == 0
    assert len(api_val.details.launched_drones.users) == 1
    assert api_drone.id in api_val.details.launched_drones.users
    # Action
    api_drone.change_drone(state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_drone(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_fighter_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_drones])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
