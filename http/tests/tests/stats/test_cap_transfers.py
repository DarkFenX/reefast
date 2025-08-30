from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionItemRemoteCps


def test_state(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=True))
    assert api_fleet_stats.remote_cps == approx(70.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=True))
    assert api_fit_stats.remote_cps == approx(70.2)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=True))
    assert api_module_stats.remote_cps.one() == approx(70.2)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=True))
    assert api_fleet_stats.remote_cps == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=True))
    assert api_fit_stats.remote_cps == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=(True, [
        StatsOptionItemRemoteCps(ignore_state=False),
        StatsOptionItemRemoteCps(ignore_state=True)])))
    assert api_module_stats.remote_cps == [0, approx(70.2)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=True))
    assert api_fleet_stats.remote_cps == approx(70.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=True))
    assert api_fit_stats.remote_cps == approx(70.2)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=True))
    assert api_module_stats.remote_cps.one() == approx(70.2)


def test_zero_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=True))
    assert api_fleet_stats.remote_cps == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=True))
    assert api_fit_stats.remote_cps == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=True))
    assert api_module_stats.remote_cps.one() == 0


def test_no_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=True))
    assert api_fleet_stats.remote_cps == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=True))
    assert api_fit_stats.remote_cps == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=True))
    assert api_module_stats.remote_cps.one() == 0


def test_item_not_loaded(client, consts):
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=True))
    assert api_fleet_stats.remote_cps == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=True))
    assert api_fit_stats.remote_cps == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=True))
    assert api_module_stats.remote_cps is None


def test_not_requested(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_cps=False))
    with check_no_field():
        api_fleet_stats.remote_cps  # noqa: B018
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_cps=False))
    with check_no_field():
        api_fit_stats.remote_cps  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_cps=False))
    with check_no_field():
        api_module_stats.remote_cps  # noqa: B018
