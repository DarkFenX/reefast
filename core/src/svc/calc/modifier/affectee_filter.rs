use crate::{ad, svc::calc::Location, ud::UItem};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AffecteeFilter {
    Direct(Location),
    Loc(Location),
    LocGrp(Location, ad::AItemGrpId),
    LocSrq(Location, ad::AItemId),
    OwnSrq(ad::AItemId),
}
impl AffecteeFilter {
    pub(super) fn from_a_affectee_filter(a_affectee_filter: &ad::AEffectAffecteeFilter, sol_item: &UItem) -> Self {
        match a_affectee_filter {
            ad::AEffectAffecteeFilter::Direct(loc) => Self::Direct(loc.into()),
            ad::AEffectAffecteeFilter::Loc(loc) => Self::Loc(loc.into()),
            ad::AEffectAffecteeFilter::LocGrp(loc, grp_id) => Self::LocGrp(loc.into(), *grp_id),
            ad::AEffectAffecteeFilter::LocSrq(loc, mod_srq) => {
                Self::LocSrq(loc.into(), get_srq_a_item_id(mod_srq, sol_item))
            }
            ad::AEffectAffecteeFilter::OwnSrq(mod_srq) => Self::OwnSrq(get_srq_a_item_id(mod_srq, sol_item)),
        }
    }
    pub(super) fn from_a_buff_affectee_filter(
        a_buff_affectee_filter: &ad::ABuffAffecteeFilter,
        sol_loc: Location,
        sol_item: &UItem,
    ) -> Self {
        match a_buff_affectee_filter {
            ad::ABuffAffecteeFilter::Direct => Self::Direct(sol_loc),
            ad::ABuffAffecteeFilter::Loc => Self::Loc(sol_loc),
            ad::ABuffAffecteeFilter::LocGrp(grp_id) => Self::LocGrp(sol_loc, *grp_id),
            ad::ABuffAffecteeFilter::LocSrq(mod_srq) => Self::LocSrq(sol_loc, get_srq_a_item_id(mod_srq, sol_item)),
        }
    }
}

fn get_srq_a_item_id(a_mod_srq: &ad::AModifierSrq, sol_item: &UItem) -> ad::AItemId {
    match a_mod_srq {
        ad::AModifierSrq::SelfRef => sol_item.get_type_id(),
        ad::AModifierSrq::ItemId(a_item_id) => *a_item_id,
    }
}
