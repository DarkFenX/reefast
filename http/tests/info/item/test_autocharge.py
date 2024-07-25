from pytest import raises


def test_autocharge(client, consts):
    eve_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_charge = client.mk_eve_item()
    eve_fighter = client.mk_eve_item(attrs={eve_attr.id: eve_charge.id}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect.id]
    assert isinstance(api_autocharge.id, str)
    with raises(AttributeError):
        api_autocharge.kind  # pylint: disable=W0104
    api_autocharge_id = api_autocharge.id
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect.id]
    assert api_autocharge.id == api_autocharge_id
    with raises(AttributeError):
        api_autocharge.kind  # pylint: disable=W0104
    api_autocharge.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_autocharge.id == api_autocharge_id
    with raises(AttributeError):
        api_autocharge.kind  # pylint: disable=W0104
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect.id]
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    api_autocharge.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect.id]
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge
    api_autocharge.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.kind == consts.ApiItemKind.autocharge


def test_invalid_reference(client, consts):
    eve_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.alloc_item_id()
    eve_fighter = client.mk_eve_item(attrs={eve_attr.id: eve_charge_id}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104


def test_no_reference(client, consts):
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_fighter = client.mk_eve_item(eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    with raises(AttributeError):
        api_fighter.autocharges  # pylint: disable=W0104
