use crate::{
    svc::calc::{Context, RawModifier},
    ud::{UFitKey, UItemKey},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct CtxModifier {
    pub(in crate::svc::calc) raw: RawModifier,
    pub(in crate::svc::calc) ctx: Context,
}
impl CtxModifier {
    pub(in crate::svc::calc) fn from_raw(raw: RawModifier) -> Self {
        Self {
            raw,
            ctx: Context::None,
        }
    }
    pub(in crate::svc::calc) fn from_raw_with_item(raw: RawModifier, ctx_item_key: UItemKey) -> Self {
        Self {
            raw,
            ctx: Context::Item(ctx_item_key),
        }
    }
    pub(in crate::svc::calc) fn from_raw_with_fit(raw: RawModifier, ctx_fit_key: UFitKey) -> Self {
        Self {
            raw,
            ctx: Context::Fit(ctx_fit_key),
        }
    }
}
