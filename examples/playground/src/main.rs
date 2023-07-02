#![allow(warnings, unused)]
#![feature(core_intrinsics)]

use std::{intrinsics::black_box, path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use itertools::Itertools;
use tracing_subscriber::prelude::*;

use rc::{ed::EveDataHandler, ModRack, OrdAddMode, SolarSystem, Src, State, VERSION};

fn setup_logger() -> () {
    let time_format_full = time::macros::format_description!(
        version = 2,
        r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
    );
    // We always log warnings and higher to stdout
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout.with_max_level(tracing::Level::TRACE))
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::new(time_format_full))
        .with_target(false)
        .pretty();
    tracing_subscriber::registry()
        .with(stdout_log)
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_default(tracing::Level::ERROR)
                .with_target("reefast_core", tracing::Level::TRACE)
                .with_target("reefast_dh_eve", tracing::Level::TRACE)
                .with_target("reefast_dh_adapted", tracing::Level::TRACE),
        )
        .init();
}

fn main() {
    setup_logger();
    let dh = Box::new(rdhe::PhbFileEdh::new("/home/dfx/Desktop/phobos_tq_en-us".into()));
    // Get some data for skills
    let grp_ids = dh
        .get_item_groups()
        .unwrap()
        .data
        .iter()
        .filter(|v| v.category_id == 16)
        .map(|v| v.id)
        .collect_vec();
    let skill_ids = dh
        .get_items()
        .unwrap()
        .data
        .iter()
        .filter(|v| grp_ids.contains(&v.group_id))
        .map(|v| v.id)
        .collect_vec();
    let mut ch = Box::new(rdha::RamJsonAdh::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/examples/playground/cache/"),
        "tq".to_string(),
    ));
    let src = Src::new(dh, ch).unwrap();
    let mut sol_sys = SolarSystem::new(src);
    let fit = sol_sys.add_fit().unwrap();
    let ship = sol_sys.set_fit_ship(fit, 11184, true).unwrap();
    for skill_id in skill_ids.iter() {
        sol_sys.add_skill(fit, skill_id.to_owned(), 5, true);
    }
    tracing::error!("starting");
    for _ in 0..1000000 {
        let anp = sol_sys
            .add_module(fit, ModRack::Low, OrdAddMode::Equip, 1306, State::Online, None)
            .unwrap();
        black_box(sol_sys.get_item_attrs(&ship.id));
        sol_sys.remove_item(&anp.id);
        black_box(sol_sys.get_item_attrs(&ship.id));
    }
    tracing::error!("done");
    // println!("{}", sol_sys.get_item_attr(&ship.id, &267).unwrap().dogma);
    // sol_sys.set_module_state(&anp.id, State::Online);
    // println!("{}", sol_sys.get_item_attr(&ship.id, &267).unwrap().dogma);
}
