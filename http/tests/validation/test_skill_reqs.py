from tests import check_no_field


def test_skill_add_remove_lower(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (2, 3)},
        api_module2.id: {eve_skill1_id: (2, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}


def test_skill_add_remove_equal(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}


def test_skill_add_remove_higher(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=4)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}


def test_skill_level_change(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (2, 3)},
        api_module2.id: {eve_skill1_id: (2, 3), eve_skill2_id: (None, 5)}}
    # Action - lower to equal
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - equal to higher
    api_skill.change_skill(level=4)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - higher to lower
    api_skill.change_skill(level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (2, 3)},
        api_module2.id: {eve_skill1_id: (2, 3), eve_skill2_id: (None, 5)}}
    # Action - lower to lower
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (0, 3)},
        api_module2.id: {eve_skill1_id: (0, 3), eve_skill2_id: (None, 5)}}
    # Action - lower to higher
    api_skill.change_skill(level=5)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - higher to higher
    api_skill.change_skill(level=4)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - higher to equal
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - equal to lower
    api_skill.change_skill(level=1)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (1, 3)},
        api_module2.id: {eve_skill1_id: (1, 3), eve_skill2_id: (None, 5)}}


def test_self_req(client, consts):
    # Unrealistic scenario, but check what happens anyway
    eve_skill_id = client.alloc_item_id()
    client.mk_eve_item(id_=eve_skill_id, srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_skill.id: {eve_skill_id: (2, 3)}}
    # Action
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_skill.id: {eve_skill_id: (0, 3)}}


def test_skill_state(client, consts):
    # Disabled skill still satisfies skill requirements
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=3, state=False)
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_item_state(client, consts):
    # Items in the lowest state still have skill requirements
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=3)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.ghost)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_item_types(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_item_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_item_id)
    api_skill = api_fit.add_skill(type_id=eve_item_id, level=0)
    api_implant = api_fit.add_implant(type_id=eve_item_id)
    api_booster = api_fit.add_booster(type_id=eve_item_id)
    api_ship = api_fit.set_ship(type_id=eve_item_id)
    api_fit.set_stance(type_id=eve_item_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_item_id)
    api_module_high = api_fit.add_mod(type_id=eve_item_id, rack=consts.ApiRack.high)
    api_module_mid = api_fit.add_mod(type_id=eve_item_id, rack=consts.ApiRack.mid)
    api_module_low = api_fit.add_mod(type_id=eve_item_id, rack=consts.ApiRack.low)
    api_fit.add_rig(type_id=eve_item_id)
    api_drone = api_fit.add_drone(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id)
    api_fit.add_fw_effect(type_id=eve_item_id)
    # Verification - characters, stances, rigs and FW effects are ignored
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_skill.id: {eve_skill_id: (None, 3)},
        api_implant.id: {eve_skill_id: (None, 3)},
        api_booster.id: {eve_skill_id: (None, 3)},
        api_ship.id: {eve_skill_id: (None, 3)},
        api_subsystem.id: {eve_skill_id: (None, 3)},
        api_module_high.id: {eve_skill_id: (None, 3)},
        api_module_mid.id: {eve_skill_id: (None, 3)},
        api_module_low.id: {eve_skill_id: (None, 3)},
        api_drone.id: {eve_skill_id: (None, 3)},
        api_fighter.id: {eve_skill_id: (None, 3)}}


def test_not_loaded_skill(client, consts):
    eve_skill_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (0, 3)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}


def test_failed_replacement(client, consts):
    # Check that failed attempt to replace skill doesn't affect validation
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=3)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_skill(type_id=eve_skill_id, level=2, status_code=409)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_skill.change_skill(level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action
    api_fit.add_skill(type_id=eve_skill_id, level=4, status_code=409)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
