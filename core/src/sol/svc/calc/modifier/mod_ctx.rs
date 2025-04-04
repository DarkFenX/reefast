use crate::sol::{
    FitId, ItemId,
    svc::calc::{Context, RawModifier},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) struct CtxModifier {
    pub(in crate::sol::svc::calc) raw: RawModifier,
    pub(in crate::sol::svc::calc) ctx: Context,
}
impl CtxModifier {
    pub(in crate::sol::svc::calc) fn from_raw(raw: RawModifier) -> Self {
        Self {
            raw,
            ctx: Context::None,
        }
    }
    pub(in crate::sol::svc::calc) fn from_raw_with_item(raw: RawModifier, ctx_item_id: ItemId) -> Self {
        Self {
            raw,
            ctx: Context::Item(ctx_item_id),
        }
    }
    pub(in crate::sol::svc::calc) fn from_raw_with_fit(raw: RawModifier, ctx_fit_id: FitId) -> Self {
        Self {
            raw,
            ctx: Context::Fit(ctx_fit_id),
        }
    }
}
