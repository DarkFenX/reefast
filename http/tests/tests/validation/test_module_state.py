from tests import check_no_field


def test_passive(client, consts):
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_module.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_online(client, consts):
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.online]}
    # Action
    api_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_active(client, consts):
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.active]}
    # Action
    api_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_overload(client, consts):
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.module_state])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
