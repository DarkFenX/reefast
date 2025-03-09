from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone1.id: 50, api_drone2.id: 100}


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_struct_id = client.mk_eve_struct(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone1.id: 50, api_drone2.id: 100}


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_drone4_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_drone5_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id])))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(250)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone2.id: 100}
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone2.id])))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(250)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone1.id: 150}
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id, api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id, api_other.id, api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id, api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone3.remove()
    api_drone4 = api_fit.add_drone(type_id=eve_drone4_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id, api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone4.remove()
    api_drone5 = api_fit.add_drone(type_id=eve_drone5_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=(True, [api_drone1.id, api_drone2.id])))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == 250.5
    assert api_val.details.drone_bandwidth.output == 125
    assert api_val.details.drone_bandwidth.users == {api_drone5.id: 0.5}


def test_modified_use(client, consts):
    # Drone bandwidth use is never modified, so the lib just uses unmodified attributes for faster
    # access to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(75)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}
    # Action
    api_implant.remove()
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(150)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}


def test_modified_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_ship.update().attrs[eve_output_attr_id].extra == approx(120)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(120)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_output_attr_id].extra == approx(180)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_base_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 120})
    eve_mutated_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 130})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(120)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=(eve_mutator_id, {eve_use_attr_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification - unrealistic scenario, but testing here detail of implementation: mutated drone
    # has different bandwidth, the lib uses bandwidth the drone had upon addition
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(145.6)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(145.6)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: approx(145.6)}
    # Action
    api_drone.change_drone(mutation={eve_use_attr_id: None})
    # Verification - unrealistic scenario, but testing here detail of implementation: mutated
    # bandwidth value does not change anything for the validation
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(130)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(145.6)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: approx(145.6)}


def test_rounding(client, consts):
    # Bandwidth shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.002})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(5.229)
    assert api_val.details.drone_bandwidth.output == approx(5.223)
    assert api_val.details.drone_bandwidth.users == {api_drone1.id: 0.002, api_drone2.id: 5.227}


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(5)
    assert api_val.details.drone_bandwidth.output is None
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 5}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(5)
    assert api_val.details.drone_bandwidth.output is None
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 5}


def test_not_loaded_user(client, consts):
    # Just check that nothing crashes, not loaded items are not supposed to even be registered
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_space)
    # Verification - items with negative and 0 use are not exposed
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(140)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone2.id: 150}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone1.id: 150}


def test_no_value_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship()
    # Make an item to ensure that output attribute is not cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(0)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}


def test_no_attr_use(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = consts.EveAttr.drone_bandwidth_used
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}


def test_no_attr_output(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = consts.EveAttr.drone_bandwidth
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output is None
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}


def test_criterion_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth.used == approx(150)
    assert api_val.details.drone_bandwidth.output == approx(125)
    assert api_val.details.drone_bandwidth.users == {api_drone.id: 150}
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    # Validation applies only to drones
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
