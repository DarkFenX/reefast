use crate::{
    ac, ec,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            opc_rep::get_remote_hull_rep_opc,
            proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
        },
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::SHIP_MOD_REMOTE_HULL_REPAIRER),
        aid: ac::effects::SHIP_MOD_REMOTE_HULL_REPAIRER,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_normal_restricted_s2s),
            remote_hull_rep_opc_getter: Some(get_remote_hull_rep_opc),
            ..
        },
        ..
    }
}
