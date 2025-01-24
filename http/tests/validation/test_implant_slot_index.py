from tests import check_no_field


def test_multiple(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_implant2 = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}
    # Action
    api_implant1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant1_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    eve_implant2_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant1_id)
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_implant3 = api_fit.add_implant(type_id=eve_implant1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant3.id])}


def test_rounding(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant1_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1.2})
    eve_implant2_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1.4})
    eve_implant3_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant1_id)
    api_implant2 = api_fit.add_implant(type_id=eve_implant2_id)
    api_fit.add_implant(type_id=eve_implant3_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}


def test_no_attr(client, consts):
    eve_slot_attr_id = consts.EveAttr.implantness
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant_id)
    api_implant2 = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}


def test_no_value(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant)
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_unloaded_user(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_state(client, consts):
    # Disabled implants still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant_id, state=False)
    api_implant2 = api_fit.add_implant(type_id=eve_implant_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}


def test_criterion_item_category(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_item_type(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.implant_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
